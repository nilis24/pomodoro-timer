use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhaseKind {
    Work,
    ShortBreak,
    LongBreak,
    ExtraWork,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanPhase {
    pub kind: PhaseKind,
    pub duration: u32,
    pub cycle: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    pub cycles: u32,
    pub work_time: u32,
    pub break_time: u32,
    pub extra_session: bool,
    pub extra_session_time: u32,
    pub used_time: u32,
    pub remaining_time: u32,
    pub phases: Vec<PlanPhase>,
}

pub trait PlanCalculator {
    fn calculate(&self) -> Plan;
}

pub struct AvailableTimePlanCalculator {
    pub available: u32,
    pub work: u32,
    pub short_break: u32,
    pub long_break: u32,
    pub long_break_every: u32,
    pub min_extra_work: u32,
    pub extra_session: bool,
}

pub struct CyclePlanCalculator {
    pub cycles: u32,
    pub work: u32,
    pub short_break: u32,
    pub long_break: u32,
    pub long_break_every: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionStatus {
    Running,
    Paused,
    Finished,
}

#[derive(Debug, Clone)]
pub struct PlanExecution {
    pub plan: Plan,
    pub phase_index: usize,
    pub remaining_seconds: u32,
    pub status: ExecutionStatus,
    last_tick: Option<Instant>,
}

impl PlanExecution {
    pub fn start(plan: Plan) -> Self {
        let remaining_seconds = plan
            .phases
            .first()
            .map(|phase| phase.duration * 60)
            .unwrap_or(0);
        let status = if remaining_seconds > 0 {
            ExecutionStatus::Running
        } else {
            ExecutionStatus::Finished
        };

        Self {
            plan,
            phase_index: 0,
            remaining_seconds,
            status,
            last_tick: Some(Instant::now()),
        }
    }

    pub fn current_phase(&self) -> Option<&PlanPhase> {
        self.plan.phases.get(self.phase_index)
    }

    pub fn play(&mut self) {
        if self.status != ExecutionStatus::Finished {
            self.status = ExecutionStatus::Running;
            self.last_tick = Some(Instant::now());
        }
    }

    pub fn pause(&mut self) {
        if self.status == ExecutionStatus::Running {
            self.status = ExecutionStatus::Paused;
            self.last_tick = None;
        }
    }

    pub fn reset(&mut self) {
        self.phase_index = 0;
        self.remaining_seconds = self
            .plan
            .phases
            .first()
            .map(|phase| phase.duration * 60)
            .unwrap_or(0);
        self.status = if self.remaining_seconds > 0 {
            ExecutionStatus::Paused
        } else {
            ExecutionStatus::Finished
        };
        self.last_tick = None;
    }

    pub fn tick(&mut self) {
        if self.status != ExecutionStatus::Running {
            return;
        }

        let now = Instant::now();
        let Some(last_tick) = self.last_tick else {
            self.last_tick = Some(now);
            return;
        };

        let elapsed_seconds = now.duration_since(last_tick).as_secs() as u32;
        if elapsed_seconds == 0 {
            return;
        }

        self.last_tick = Some(last_tick + std::time::Duration::from_secs(elapsed_seconds as u64));
        self.advance(elapsed_seconds);
    }

    fn advance(&mut self, mut seconds: u32) {
        while seconds > 0 && self.status == ExecutionStatus::Running {
            if seconds < self.remaining_seconds {
                self.remaining_seconds -= seconds;
                break;
            }

            seconds -= self.remaining_seconds;
            self.phase_index += 1;

            if let Some(phase) = self.current_phase() {
                self.remaining_seconds = phase.duration * 60;
            } else {
                self.remaining_seconds = 0;
                self.status = ExecutionStatus::Finished;
                self.last_tick = None;
            }
        }
    }
}

pub fn calculate_plan(
    available: u32,
    work: u32,
    short_break: u32,
    long_break: u32,
    long_break_every: u32,
    min_extra_work: u32,
    extra_session: bool,
) -> Plan {
    AvailableTimePlanCalculator {
        available,
        work,
        short_break,
        long_break,
        long_break_every,
        min_extra_work,
        extra_session,
    }
    .calculate()
}

pub fn calculate_cycle_plan(
    cycles: u32,
    work: u32,
    short_break: u32,
    long_break: u32,
    long_break_every: u32,
) -> Plan {
    CyclePlanCalculator {
        cycles,
        work,
        short_break,
        long_break,
        long_break_every,
    }
    .calculate()
}

impl PlanCalculator for AvailableTimePlanCalculator {
    fn calculate(&self) -> Plan {
        calculate_available_time_plan(self)
    }
}

impl PlanCalculator for CyclePlanCalculator {
    fn calculate(&self) -> Plan {
        calculate_cycles_plan(self)
    }
}

fn calculate_available_time_plan(calculator: &AvailableTimePlanCalculator) -> Plan {
    let available = calculator.available;
    let work = calculator.work;
    let short_break = calculator.short_break;
    let long_break = calculator.long_break;
    let long_break_every = calculator.long_break_every;
    let min_extra_work = calculator.min_extra_work;
    let extra_session = calculator.extra_session;

    if available == 0 {
        return Plan {
            cycles: 0,
            work_time: 0,
            break_time: 0,
            extra_session,
            extra_session_time: 0,
            used_time: 0,
            remaining_time: 0,
            phases: Vec::new(),
        };
    }

    if available < work {
        return Plan {
            cycles: 1,
            work_time: available,
            break_time: 0,
            extra_session: false,
            extra_session_time: 0,
            used_time: available,
            remaining_time: 0,
            phases: vec![PlanPhase {
                kind: PhaseKind::Work,
                duration: available,
                cycle: 1,
            }],
        };
    }

    let mut used = 0;
    let mut cycles = 0;
    let mut total_break = 0;
    let mut total_work = 0;
    let mut extra_session_time = 0;
    let mut phases = Vec::new();

    while used + work <= available {
        used += work;
        total_work += work;
        cycles += 1;
        phases.push(PlanPhase {
            kind: PhaseKind::Work,
            duration: work,
            cycle: cycles,
        });

        let (next_break, next_break_kind) =
            if long_break_every > 0 && cycles % long_break_every == 0 {
                (long_break, PhaseKind::LongBreak)
            } else {
                (short_break, PhaseKind::ShortBreak)
            };

        if used + next_break + work <= available {
            used += next_break;
            total_break += next_break;
            phases.push(PlanPhase {
                kind: next_break_kind,
                duration: next_break,
                cycle: cycles,
            });
        } else if extra_session && used + next_break + min_extra_work <= available {
            used += next_break;
            total_break += next_break;
            phases.push(PlanPhase {
                kind: next_break_kind,
                duration: next_break,
                cycle: cycles,
            });

            extra_session_time = available - used;
            used += extra_session_time;
            total_work += extra_session_time;
            cycles += 1;
            phases.push(PlanPhase {
                kind: PhaseKind::ExtraWork,
                duration: extra_session_time,
                cycle: cycles,
            });
            break;
        } else {
            break;
        }
    }

    let remaining_time = available - used;

    Plan {
        cycles,
        work_time: total_work,
        break_time: total_break,
        extra_session: extra_session_time > 0,
        extra_session_time,
        used_time: used,
        remaining_time,
        phases,
    }
}

fn calculate_cycles_plan(calculator: &CyclePlanCalculator) -> Plan {
    let mut phases = Vec::new();
    let mut total_break = 0;
    let mut total_work = 0;

    for cycle in 1..=calculator.cycles {
        total_work += calculator.work;
        phases.push(PlanPhase {
            kind: PhaseKind::Work,
            duration: calculator.work,
            cycle,
        });

        if cycle == calculator.cycles {
            continue;
        }

        let (break_time, break_kind) =
            if calculator.long_break_every > 0 && cycle % calculator.long_break_every == 0 {
                (calculator.long_break, PhaseKind::LongBreak)
            } else {
                (calculator.short_break, PhaseKind::ShortBreak)
            };

        total_break += break_time;
        phases.push(PlanPhase {
            kind: break_kind,
            duration: break_time,
            cycle,
        });
    }

    Plan {
        cycles: calculator.cycles,
        work_time: total_work,
        break_time: total_break,
        extra_session: false,
        extra_session_time: 0,
        used_time: total_work + total_break,
        remaining_time: 0,
        phases,
    }
}

#[cfg(test)]
mod tests {
    use super::{PhaseKind, calculate_cycle_plan, calculate_plan};

    #[test]
    fn uses_available_time_as_one_short_work_cycle() {
        let plan = calculate_plan(20, 25, 5, 15, 4, 10, true);

        assert_eq!(plan.cycles, 1);
        assert_eq!(plan.work_time, 20);
        assert_eq!(plan.break_time, 0);
        assert!(!plan.extra_session);
        assert_eq!(plan.used_time, 20);
        assert_eq!(plan.remaining_time, 0);
    }

    #[test]
    fn keeps_remaining_time_without_extra_session() {
        let plan = calculate_plan(70, 25, 5, 15, 4, 10, false);

        assert_eq!(plan.cycles, 2);
        assert_eq!(plan.work_time, 50);
        assert_eq!(plan.break_time, 5);
        assert_eq!(plan.used_time, 55);
        assert_eq!(plan.remaining_time, 15);
    }

    #[test]
    fn adds_remaining_time_as_extra_session_when_requested() {
        let plan = calculate_plan(70, 25, 5, 15, 4, 10, true);

        assert_eq!(plan.cycles, 3);
        assert_eq!(plan.work_time, 60);
        assert_eq!(plan.break_time, 10);
        assert!(plan.extra_session);
        assert_eq!(plan.extra_session_time, 10);
        assert_eq!(plan.used_time, 70);
        assert_eq!(plan.remaining_time, 0);
    }

    #[test]
    fn does_not_add_extra_session_without_room_for_the_next_break() {
        let plan = calculate_plan(120, 25, 5, 15, 4, 10, true);

        assert_eq!(plan.cycles, 4);
        assert_eq!(plan.work_time, 100);
        assert_eq!(plan.break_time, 15);
        assert!(!plan.extra_session);
        assert_eq!(plan.extra_session_time, 0);
        assert_eq!(plan.used_time, 115);
        assert_eq!(plan.remaining_time, 5);
    }

    #[test]
    fn does_not_add_extra_session_below_minimum_work_time() {
        let plan = calculate_plan(61, 25, 5, 15, 4, 10, true);

        assert_eq!(plan.cycles, 2);
        assert_eq!(plan.work_time, 50);
        assert_eq!(plan.break_time, 5);
        assert!(!plan.extra_session);
        assert_eq!(plan.extra_session_time, 0);
        assert_eq!(plan.used_time, 55);
        assert_eq!(plan.remaining_time, 6);
    }

    #[test]
    fn uses_configured_long_break_frequency() {
        let plan = calculate_plan(100, 25, 5, 15, 2, 10, false);

        assert_eq!(plan.cycles, 3);
        assert_eq!(plan.work_time, 75);
        assert_eq!(plan.break_time, 20);
        assert_eq!(plan.used_time, 95);
        assert_eq!(plan.remaining_time, 5);
    }

    #[test]
    fn calculates_plan_for_fixed_number_of_cycles_without_trailing_break() {
        let plan = calculate_cycle_plan(4, 25, 5, 15, 4);

        assert_eq!(plan.cycles, 4);
        assert_eq!(plan.work_time, 100);
        assert_eq!(plan.break_time, 15);
        assert_eq!(plan.used_time, 115);
        assert_eq!(plan.remaining_time, 0);
        assert_eq!(plan.phases.len(), 7);
        assert_eq!(plan.phases.last().unwrap().kind, PhaseKind::Work);
    }

    #[test]
    fn uses_long_break_between_fixed_cycles() {
        let plan = calculate_cycle_plan(5, 25, 5, 15, 2);

        assert_eq!(plan.work_time, 125);
        assert_eq!(plan.break_time, 40);
        assert_eq!(plan.used_time, 165);
        assert_eq!(plan.phases[3].kind, PhaseKind::LongBreak);
    }
}
