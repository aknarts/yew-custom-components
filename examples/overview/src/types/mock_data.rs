use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct Data {
    pub data: Vec<(i32, String, i64)>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            data: vec![
                (0, String::from("Big Brown Fox"), 0),
                (1, String::from("Yellow Brick Road"), 5),
                (2, String::from("Lorem Ipsum"), 6),
                (3, String::from("Quick Brown Fox"), 6),
                (4, String::from("Lazy Dog"), 8),
                (5, String::from("Jumps Over"), 10),
                (6, String::from("The Moon"), 12),
                (7, String::from("Sunset Boulevard"), 14),
                (8, String::from("Rust Programming"), 16),
                (9, String::from("GitHub Copilot"), 18),
                (10, String::from("Artificial Intelligence"), 20),
                (11, String::from("Machine Learning"), 22),
                (12, String::from("Deep Learning"), 24),
                (13, String::from("Neural Networks"), 26),
                (14, String::from("Data Science"), 28),
                (15, String::from("Web Development"), 30),
                (16, String::from("Mobile Development"), 32),
                (17, String::from("Game Development"), 34),
                (18, String::from("Virtual Reality"), 36),
                (19, String::from("Augmented Reality"), 38),
                (20, String::from("Cloud Computing"), 40),
                (21, String::from("Internet of Things"), 42),
                (22, String::from("Cybersecurity"), 44)],
        }
    }
}

pub enum DataActions {
    AddData((i32, String, i64)),
    #[allow(dead_code)]
    RemoveData(i32),
    RandomizeData,
}

impl yew::Reducible for Data {
    type Action = DataActions;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut new = (*self).clone();
        let mut rng = rand::thread_rng();
        match action {
            DataActions::AddData(data) => {
                new.data.push(data);
            }
            DataActions::RemoveData(id) => {
                new.data.retain(|(i, _, _)| i != &id);
            }
            DataActions::RandomizeData => {
                for (_, _, x) in new.data.iter_mut() {
                    *x = (*x) + rng.gen_range(-5..5);
                }
            }
        }
        std::rc::Rc::new(new)
    }
}
