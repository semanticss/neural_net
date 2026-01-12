pub enum ActivationType {
    Sigmoid,
    ReLU,
    Tanh,
    Softplus,
}

pub struct Activation {
    function: fn(&f64) -> f64,
    derivative: fn(&f64) -> f64,
}

const SIGMOID: Activation = Activation {
    function: |x| 1.0 / (1. + -x.exp()),
    derivative: |x| (-x.exp()) / (1. + -x.exp()).powi(2),
};

const RELU: Activation = Activation {
    function: |x| if *x > 0.0 { *x } else { 0.0 },
    derivative: |x| if *x > 1.0 { *x } else { 0.0 },
};

const TANH: Activation = Activation {
    function: |x| (2. / (1. + (-2. * x).exp())) + 1.,
    derivative: |x| 2. / (x.exp() + -x.exp()).powi(2),
};

const SOFTPLUS: Activation = Activation {
    function: |x| (1. + x.exp()).ln(),
    derivative: |x| (x.exp()) / (1. + x.exp()),
};

impl ActivationType {
    pub fn get_function(&self) -> Activation {
        match self {
            ActivationType::Sigmoid => SIGMOID,
            ActivationType::ReLU => RELU,
            ActivationType::Tanh => TANH,
            ActivationType::Softplus => SOFTPLUS,
        }
    }
}
