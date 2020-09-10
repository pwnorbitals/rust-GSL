//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Mode {
    PrecDouble,
    PrecSingle,
    PrecApprox,
}

impl Into<sys::gsl_mode_t> for Mode {
    fn into(self) -> sys::gsl_mode_t {
        match self {
            Mode::PrecDouble => 0,
            Mode::PrecSingle => 1,
            Mode::PrecApprox => 2,
        }
    }
}

impl From<sys::gsl_mode_t> for Mode {
    fn from(v: sys::gsl_mode_t) -> Mode {
        match v {
            0 => Mode::PrecDouble,
            1 => Mode::PrecSingle,
            2 => Mode::PrecApprox,
            _ => panic!("Unknown VegasMode value"),
        }
    }
}

/// A type for results generated by GSL functions where `Err` is `enums::Value`.
pub type GSLResult<T> = ::std::result::Result<T, Value>;

impl ::std::convert::From<Value> for GSLResult<()> {
    fn from(v: Value) -> Self {
        match v {
            Value::Success => Ok(()),
            e => Err(e),
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Value {
    Success,
    Failure,
    /// iteration has not converged
    Continue,
    /// input domain error, e.g sqrt(-1)
    Domain,
    /// output range error, e.g. exp(1e100)
    Range,
    /// invalid pointer
    Fault,
    /// invalid argument supplied by user
    Invalid,
    /// generic failure
    Failed,
    /// factorization failed
    Factorization,
    /// sanity check failed - shouldn't happen
    Sanity,
    /// malloc failed
    NoMemory,
    /// problem with user-supplied function
    BadFunction,
    /// iterative process is out of control
    RunAway,
    /// exceeded max number of iterations
    MaxIteration,
    /// tried to divide by zero
    ZeroDiv,
    /// user specified an invalid tolerance
    BadTolerance,
    /// failed to reach the specified tolerance
    Tolerance,
    /// underflow
    UnderFlow,
    /// overflow
    OverFlow,
    /// loss of accuracy
    Loss,
    /// failed because of roundoff error
    Round,
    /// matrix, vector lengths are not conformant
    BadLength,
    /// matrix not square
    NotSquare,
    /// apparent singularity detected
    Singularity,
    /// integral or series is divergent
    Diverge,
    /// requested feature is not supported by the hardware
    Unsupported,
    /// requested feature not (yet) implemented
    Unimplemented,
    /// cache limit exceeded
    Cache,
    /// table limit exceeded
    Table,
    /// iteration is not making progress towards solution
    NoProgress,
    /// jacobian evaluations are not improving the solution
    NoProgressJacobian,
    /// cannot reach the specified tolerance in F
    ToleranceF,
    /// cannot reach the specified tolerance in X
    ToleranceX,
    /// cannot reach the specified tolerance in gradient
    ToleranceG,
    /// cannot reach the specified tolerance in gradient
    EOF,
    /// Unknown value.
    Unknown(i32),
}

impl Value {
    pub fn is_success(self) -> bool {
        self == Self::Success
    }
}

impl Into<libc::c_int> for Value {
    fn into(self) -> libc::c_int {
        match self {
            Value::Success => 0,
            Value::Failure => -1,
            Value::Continue => -2,
            Value::Domain => 1,
            Value::Range => 2,
            Value::Fault => 3,
            Value::Invalid => 4,
            Value::Failed => 5,
            Value::Factorization => 6,
            Value::Sanity => 7,
            Value::NoMemory => 8,
            Value::BadFunction => 9,
            Value::RunAway => 10,
            Value::MaxIteration => 11,
            Value::ZeroDiv => 12,
            Value::BadTolerance => 13,
            Value::Tolerance => 14,
            Value::UnderFlow => 15,
            Value::OverFlow => 16,
            Value::Loss => 17,
            Value::Round => 18,
            Value::BadLength => 19,
            Value::NotSquare => 20,
            Value::Singularity => 21,
            Value::Diverge => 22,
            Value::Unsupported => 23,
            Value::Unimplemented => 24,
            Value::Cache => 25,
            Value::Table => 26,
            Value::NoProgress => 27,
            Value::NoProgressJacobian => 28,
            Value::ToleranceF => 29,
            Value::ToleranceX => 30,
            Value::ToleranceG => 31,
            Value::EOF => 32,
            Value::Unknown(x) => x,
        }
    }
}

impl From<libc::c_int> for Value {
    fn from(v: libc::c_int) -> Value {
        match v {
            -2 => Value::Continue,
            -1 => Value::Failure,
            0 => Value::Success,
            1 => Value::Domain,
            2 => Value::Range,
            3 => Value::Fault,
            4 => Value::Invalid,
            5 => Value::Failed,
            6 => Value::Factorization,
            7 => Value::Sanity,
            8 => Value::NoMemory,
            9 => Value::BadFunction,
            10 => Value::RunAway,
            11 => Value::MaxIteration,
            12 => Value::ZeroDiv,
            13 => Value::BadTolerance,
            14 => Value::Tolerance,
            15 => Value::UnderFlow,
            16 => Value::OverFlow,
            17 => Value::Loss,
            18 => Value::Round,
            19 => Value::BadLength,
            20 => Value::NotSquare,
            21 => Value::Singularity,
            22 => Value::Diverge,
            23 => Value::Unsupported,
            24 => Value::Unimplemented,
            25 => Value::Cache,
            26 => Value::Table,
            27 => Value::NoProgress,
            28 => Value::NoProgressJacobian,
            29 => Value::ToleranceF,
            30 => Value::ToleranceX,
            31 => Value::ToleranceG,
            32 => Value::EOF,
            x => Value::Unknown(x),
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum EigenSort {
    /// ascending order in numerical value
    ValAsc,
    /// descending order in numerical value
    ValDesc,
    /// ascending order in magnitude
    AbsAsc,
    /// descending order in magnitude
    AbsDesc,
}

impl Into<libc::c_int> for EigenSort {
    fn into(self) -> libc::c_int {
        match self {
            EigenSort::ValAsc => 0,
            EigenSort::ValDesc => 1,
            EigenSort::AbsAsc => 2,
            EigenSort::AbsDesc => 3,
        }
    }
}

impl From<libc::c_int> for EigenSort {
    fn from(v: libc::c_int) -> EigenSort {
        match v {
            0 => EigenSort::ValAsc,
            1 => EigenSort::ValDesc,
            2 => EigenSort::AbsAsc,
            3 => EigenSort::AbsDesc,
            _ => panic!("Unknown EigenSort value"),
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
/// this gives the sign in the formula
///
/// ```text
/// h(f) = \sum x(t) exp(+/- 2 pi i f t)
/// ```
///
/// where - is the forward transform direction and + the inverse direction
pub enum FftDirection {
    Forward,
    Backward,
}

impl Into<libc::c_int> for FftDirection {
    fn into(self) -> libc::c_int {
        match self {
            FftDirection::Forward => -1,
            FftDirection::Backward => 1,
        }
    }
}

impl From<libc::c_int> for FftDirection {
    fn from(v: libc::c_int) -> FftDirection {
        match v {
            -1 => FftDirection::Forward,
            1 => FftDirection::Backward,
            _ => panic!("Unknown FftDirection value"),
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
/// The low-level integration rules in QUADPACK are identified by small integers (1-6). We'll use symbolic constants to refer to them.
pub enum GaussKonrodRule {
    /// 15 point Gauss-Kronrod rule
    Gauss15,
    /// 21 point Gauss-Kronrod rule
    Gauss21,
    /// 31 point Gauss-Kronrod rule
    Gauss31,
    /// 41 point Gauss-Kronrod rule
    Gauss41,
    /// 51 point Gauss-Kronrod rule
    Gauss51,
    /// 61 point Gauss-Kronrod rule
    Gauss61,
}

impl Into<libc::c_int> for GaussKonrodRule {
    fn into(self) -> libc::c_int {
        match self {
            GaussKonrodRule::Gauss15 => 1,
            GaussKonrodRule::Gauss21 => 2,
            GaussKonrodRule::Gauss31 => 3,
            GaussKonrodRule::Gauss41 => 4,
            GaussKonrodRule::Gauss51 => 5,
            GaussKonrodRule::Gauss61 => 6,
        }
    }
}

impl From<libc::c_int> for GaussKonrodRule {
    fn from(v: libc::c_int) -> GaussKonrodRule {
        match v {
            1 => GaussKonrodRule::Gauss15,
            2 => GaussKonrodRule::Gauss21,
            3 => GaussKonrodRule::Gauss31,
            4 => GaussKonrodRule::Gauss41,
            5 => GaussKonrodRule::Gauss51,
            6 => GaussKonrodRule::Gauss61,
            _ => panic!("Unknown GaussKonrodRule value"),
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
/// Used by workspace for QAWO integrator
pub enum IntegrationQawo {
    Cosine,
    Sine,
}

impl Into<libc::c_int> for IntegrationQawo {
    fn into(self) -> libc::c_int {
        match self {
            IntegrationQawo::Cosine => 0,
            IntegrationQawo::Sine => 1,
        }
    }
}

impl From<libc::c_int> for IntegrationQawo {
    fn from(v: libc::c_int) -> IntegrationQawo {
        match v {
            0 => IntegrationQawo::Cosine,
            1 => IntegrationQawo::Sine,
            _ => panic!("Unknown IntegrationQawo value"),
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
/// Used by VegasMonteCarlo struct
///
/// The possible choices are GSL_VEGAS_MODE_IMPORTANCE, GSL_VEGAS_MODE_
/// STRATIFIED, GSL_VEGAS_MODE_IMPORTANCE_ONLY. This determines whether vegas
/// will use importance sampling or stratified sampling, or whether it can pick on
/// its own. In low dimensions vegas uses strict stratified sampling (more precisely,
/// stratified sampling is chosen if there are fewer than 2 bins per box).
pub enum VegasMode {
    Importance,
    ImportanceOnly,
    Stratified,
}

impl Into<libc::c_int> for VegasMode {
    fn into(self) -> libc::c_int {
        match self {
            VegasMode::Importance => 1,
            VegasMode::ImportanceOnly => 0,
            VegasMode::Stratified => -1,
        }
    }
}

impl From<libc::c_int> for VegasMode {
    fn from(v: libc::c_int) -> VegasMode {
        match v {
            1 => VegasMode::Importance,
            0 => VegasMode::ImportanceOnly,
            -1 => VegasMode::Stratified,
            _ => panic!("Unknown VegasMode value"),
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
/// Possible return values for an hadjust() evolution method for ordinary differential equations
pub enum ODEiv {
    /// step was increased
    Inc,
    /// step unchanged
    Nil,
    /// step decreased
    Dec,
}

impl Into<libc::c_int> for ODEiv {
    fn into(self) -> libc::c_int {
        match self {
            ODEiv::Inc => 1,
            ODEiv::Nil => 0,
            ODEiv::Dec => -1,
        }
    }
}

impl From<libc::c_int> for ODEiv {
    fn from(v: libc::c_int) -> ODEiv {
        match v {
            1 => ODEiv::Inc,
            0 => ODEiv::Nil,
            -1 => ODEiv::Dec,
            _ => panic!("Unknown ODEiv value"),
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum WaveletDirection {
    Forward,
    Backward,
}

impl Into<libc::c_int> for WaveletDirection {
    fn into(self) -> libc::c_int {
        match self {
            WaveletDirection::Forward => 1,
            WaveletDirection::Backward => -1,
        }
    }
}

impl From<libc::c_int> for WaveletDirection {
    fn from(v: libc::c_int) -> WaveletDirection {
        match v {
            1 => WaveletDirection::Forward,
            -1 => WaveletDirection::Backward,
            _ => panic!("Unknown WaveletDirection value"),
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum SfLegendreNorm {
    Schmidt,
    SphericalHarmonic,
    Full,
    None,
}

impl Into<libc::c_int> for SfLegendreNorm {
    fn into(self) -> libc::c_int {
        match self {
            SfLegendreNorm::Schmidt => 0,
            SfLegendreNorm::SphericalHarmonic => 1,
            SfLegendreNorm::Full => 2,
            SfLegendreNorm::None => 3,
        }
    }
}

impl From<libc::c_int> for SfLegendreNorm {
    fn from(v: libc::c_int) -> SfLegendreNorm {
        match v {
            0 => SfLegendreNorm::Schmidt,
            1 => SfLegendreNorm::SphericalHarmonic,
            2 => SfLegendreNorm::Full,
            3 => SfLegendreNorm::None,
            _ => panic!("Unknown SfLegendreNorm value"),
        }
    }
}
