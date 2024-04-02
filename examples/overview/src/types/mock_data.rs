use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct Data {
    pub data: Vec<(i32, String, i64)>,
}

impl Default for Data {
    fn default() -> Self {
        Self { data: vec![(0, String::from("Big Brown Fox"), 0),
                          (1, String::from("Yellow Brick Road"), 5),
                          (2, String::from("Lorem Ipsum"), 6)] }
    }
}

pub enum DataActions {
    AddData((i32, String, i64)),
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
                    *x = (*x)+ rng.gen_range(-5..5);
                }
            }
        }
        std::rc::Rc::new(new)
    }
}
