/// Represents the different methods used in the Gulag Cleaner application.
pub enum Method {
    /// The Wuolah method, which takes a vector of vectors of tuples containing unsigned integers and unsigned shorts,
    /// and a vector of unsigned integers as parameters.
    Wuolah(Vec<Vec<(u32, u16)>>, Vec<u32>),
    /// The StuDocu method, which takes a vector of vectors of tuples containing unsigned integers and unsigned shorts
    /// as a parameter.
    StuDocu(Vec<Vec<(u32, u16)>>),
    /// The Naive method, which does not take any parameters.
    Naive,
}
