#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    pub cycles: u32,
    pub work_time: u32,
    pub break_time: u32,
    pub extra_session: bool,
    pub extra_session_time: u32,
    pub used_time: u32,
    pub remaining_time: u32,
}

pub fn calculate_plan(
    available: u32,
    work: u32,
    short_break: u32,
    long_break: u32,
    long_break_every: u32,
    extra_session: bool,
) -> Plan {
    if available == 0 {
        return Plan {
            cycles: 0,
            work_time: 0,
            break_time: 0,
            extra_session,
            extra_session_time: 0,
            used_time: 0,
            remaining_time: 0,
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
        };
    }

    let mut used = 0;
    let mut cycles = 0;
    let mut total_break = 0;
    let mut total_work = 0;
    let mut extra_session_time = 0;

    while used + work <= available {
        used += work;
        total_work += work;
        cycles += 1;

        let next_break = if long_break_every > 0 && cycles % long_break_every == 0 {
            long_break
        } else {
            short_break
        };

        if used + next_break + work <= available {
            used += next_break;
            total_break += next_break;
        } else if extra_session && used + next_break < available {
            used += next_break;
            total_break += next_break;

            extra_session_time = available - used;
            used += extra_session_time;
            total_work += extra_session_time;
            cycles += 1;
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
    }
}

#[cfg(test)]
mod tests {
    use super::calculate_plan;

    #[test]
    fn uses_available_time_as_one_short_work_cycle() {
        let plan = calculate_plan(20, 25, 5, 15, 4, true);

        assert_eq!(plan.cycles, 1);
        assert_eq!(plan.work_time, 20);
        assert_eq!(plan.break_time, 0);
        assert!(!plan.extra_session);
        assert_eq!(plan.used_time, 20);
        assert_eq!(plan.remaining_time, 0);
    }

    #[test]
    fn keeps_remaining_time_without_extra_session() {
        let plan = calculate_plan(70, 25, 5, 15, 4, false);

        assert_eq!(plan.cycles, 2);
        assert_eq!(plan.work_time, 50);
        assert_eq!(plan.break_time, 5);
        assert_eq!(plan.used_time, 55);
        assert_eq!(plan.remaining_time, 15);
    }

    #[test]
    fn adds_remaining_time_as_extra_session_when_requested() {
        let plan = calculate_plan(70, 25, 5, 15, 4, true);

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
        let plan = calculate_plan(120, 25, 5, 15, 4, true);

        assert_eq!(plan.cycles, 4);
        assert_eq!(plan.work_time, 100);
        assert_eq!(plan.break_time, 15);
        assert!(!plan.extra_session);
        assert_eq!(plan.extra_session_time, 0);
        assert_eq!(plan.used_time, 115);
        assert_eq!(plan.remaining_time, 5);
    }
}
