// Once you've got the documentation here, run `cargo doc --no-deps --open` and take a look!

//! # Pumpkin
//!
//! A pumpkin is a cultivar of winter squash that is round with smooth, slightly ribbed skin, and
//! is most often deep yellow to orange in coloration.[1] The thick shell contains the seeds and
//! pulp. The name is most commonly used for cultivars of Cucurbita pepo, but some cultivars of
//! Cucurbita maxima, C. argyrosperma, and C. moschata with similar appearance are also sometimes
//! called "pumpkins".[1]

/// Big orange thing
/// ![Big orange thing](https://upload.wikimedia.org/wikipedia/commons/thumb/5/5c/FrenchMarketPumpkinsB.jpg/700px-FrenchMarketPumpkinsB.jpg)
///
/// # Recipes
///
/// // ... coming soon
///
/// # Examples
///
/// ```
/// let pumpkin = Pumpkin::new({roundness:1.0, orangeness:1});
/// ```
pub struct Pumpkin {
    // Pumpkin's roundness expressed as a percentage 
    pub roundness: f32,
    // Pumpkin's orangeness expressed as a number ranging from [8, 27]
    pub orangeness: i32,
}

impl Pumpkin {
    /// Smash the pumpkin. A smashed pumpkin is gone forever.
    pub fn smash(self) {}
}

// 5. Document that 
// - Link to the Pumpkin struct in your description

/// `BURNT_ORANGE` is used to to give the right "orangeness" of our Pumpkin in
/// `Pumpkin::orangeness`
pub const BURNT_ORANGE: i32 = 13;

// Challenge: Find the option to pass to `cargo doc` so that documentation for this private item
// gets generated as well.  Hint: `cargo doc -h` will show you all the relevant options.

/// For internal use only. In fact, this documentation is so private that it won't be generated.
/// At least not by default. But if you pass the correct option in, it will magically appear!
#[allow(dead_code)] // to silence the warning
enum PrivateEnum {
    /// For Halloween. To be lit by candlelight.
    JackOLantern,
    /// For dessert during North American winter holidays.
    PumpkinPie,
}
