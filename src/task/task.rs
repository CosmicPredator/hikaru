use crate::task::task_schedule::TaskSchedule;

pub struct Task {
    name: String,
    max_retires: u32,
    schedule: TaskSchedule
}



