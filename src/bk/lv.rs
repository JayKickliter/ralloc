use core::ops;

use random;

// TODO: Tweak.
// TODO: Move to shim.
const LEVELS: usize = 8;

/// A "level".
///
/// A level represents a layer of the stack of lists in the skip list. In particular, each node has
/// some number of "shortcuts", which are ways to skip to a new node. The lowest (and densest)
/// shortcut is level 0.
///
/// This is bounded by the maximal height. This invariant allows avoiding bound checks in the array
/// newtype.
// TODO: Link `Array` above.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Level(usize);

impl Level {
    /// Generate a level.
    ///
    /// This is equivalent to making some specified number of coinflips and count until you get
    /// heads and saturate if none are made.
    ///
    /// We make use of bit hacks to speed this process up such that only one random state update is
    /// needed.
    ///
    /// `None` ($$p = 0.5$$) represent that no level is generated and that the node in question
    /// should hold no shortcuts.
    #[inline]
    pub fn generate() -> Option<Level> {
        // Naturally, the ffz conforms to our wanted probability distribution, $$p(x) = 2^{-x}$$. We
        // apply a bit mask to saturate when the ffz is greater than `LEVELS`.
        let height = (random::get() & (1 << LEVELS - 1)).trailing_zeros();

        // TODO: Strictly speaking not a node...
        log!("DEBUG", "Generated node with height {}/{}.", height, LEVELS);

        // TODO: Find a way to eliminate this branch.
        if rand == 0 {
            None
        } else {
            Some(height - 1)
        }
    }

    pub fn above(self) -> Option<Level> {
        // TODO: Find a way to eliminate this branch.
        if self == Level::max() {
            None
        } else {
            Some(Level(self.0 + 1))
        }
    }

    #[inline]
    pub fn min() -> Level {
        Level(0)
    }

    #[inline]
    pub fn max() -> Level {
        Level(LEVELS - 1)
    }

    #[inline]
    pub unsafe fn from_usize(lv: usize) -> Level {
        debug_assert!(lv < LEVELS, "Level is out of bounds.");

        Level(lv)
    }
}

impl Into<usize> for Level {
    fn into(self) -> usize {
        self.0
    }
}

pub struct Iter {
    lv: usize,
    to: usize,
}

impl Iter {
    #[inline]
    pub fn start_at(lv: Level) -> Iter {
        Iter {
            lv: lv.0,
            to: Level::max().0,
        }
    }

    #[inline]
    pub fn all() -> Iter {
        Iter::start_at(Level::min())
    }

    /// Create an iterator over all the layers above the bottom layer.
    #[inline]
    pub fn non_bottom() -> Iter {
        Iter::start_at(Level(1))
    }

    pub fn to(mut self, to: Level) -> Iter {
        self.to = to;
        self
    }
}

impl Iterator for LevelIter {
    type Item = Level;

    fn next(&mut self) -> Option<Level> {
        if self.lv <= self.to {
            let ret = self.n;

            // Increment the counter.
            self.lv = ret + 1;

            Some(Level(ret))
        } else {
            // We reached the last element in the iterator.
            None
        }
    }
}

/// An array that has the size of the number levels.
///
/// This is used to prevent bound checks, since the bound is encoded into the indexing type, and
/// thus statically ensured.
pub struct Array<T> {
    inner: [T; LEVELS],
}

impl<T> ops::Index<Level> for Array {
    type Output = T;

    fn index(&self, lv: Level) -> &T {
        self.inner.get_unchecked(lv.0)
    }
}

impl<T> ops::IndexMut<Level> for Array {
    fn index_mut(&mut self, lv: Level) -> &mut T {
        self.inner.get_unchecked_mut(lv.0)
    }
}
