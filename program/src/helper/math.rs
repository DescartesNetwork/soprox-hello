use std::convert::TryFrom;

///
/// Calculation precision
///
pub const PRECISION: u64 = 1000000000; // 10^9
pub const DOUBLE_PRECISION: u64 = 1000000000000000000; // 10^18

///
/// Implement square root for u128
///
pub trait U128Roots {
  fn sqrt(self) -> Self;
  fn to_u64(self) -> Option<u64>;
}

impl U128Roots for u128 {
  ///
  /// Babylonian method (with a selectively initial guesses)
  /// O(log(log(n))) for complexity
  ///
  fn sqrt(self) -> Self {
    if self < 2 {
      return self;
    }

    let bits = (128 - self.leading_zeros() + 1) / 2;
    let mut start = 1 << (bits - 1);
    let mut end = 1 << (bits + 1);
    while start < end {
      end = (start + end) / 2;
      start = self / end;
    }
    end
  }

  ///
  /// Cast u128 to u64
  ///
  fn to_u64(self) -> Option<u64> {
    u64::try_from(self).ok()
  }
}

///
/// Implement square root for u64
///
pub trait U64Roots {
  fn sqrt(self) -> Self;
  fn to_u128(self) -> Option<u128>;
}

impl U64Roots for u64 {
  ///
  /// Babylonian method (with a selectively initial guesses)
  /// O(log(log(n))) for complexity
  ///
  fn sqrt(self) -> Self {
    if self < 2 {
      return self;
    }

    let bits = (64 - self.leading_zeros() + 1) / 2;
    let mut start = 1 << (bits - 1);
    let mut end = 1 << (bits + 1);
    while start < end {
      end = (start + end) / 2;
      start = self / end;
    }
    end
  }

  ///
  /// Cast u64 to u128
  ///
  fn to_u128(self) -> Option<u128> {
    u128::try_from(self).ok()
  }
}
