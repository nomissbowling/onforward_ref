//! ops
//!

/// onforward_ref_unop without Copy derive
#[macro_export]
macro_rules! onforward_ref_unop {
  (impl $imp:ident, $method:ident for $t:ty) => {
    /// impl '$imp' for '$t'
    impl $imp for $t {
      type Output = Self;

      /// '$imp' '$t'
      #[inline]
      fn $method(self) -> Self {
        $imp::$method(&self)
      }
    }

    crate::ops::onforward_ref_mut_unop!{impl $imp, $method for $t}
  };
}
pub use onforward_ref_unop;

/// onforward_ref_mut_unop without Copy derive
#[macro_export]
macro_rules! onforward_ref_mut_unop {
  (impl $imp:ident, $method:ident for $t:ty) => {
    /// impl '$imp' for '&amp;mut $t'
    impl<'a> $imp for &'a mut $t {
      type Output = <$t as $imp>::Output;

      /// '$imp' '&amp;mut $t'
      #[inline]
      fn $method(self) -> <$t as $imp>::Output {
        // '&*self' means cast '&mut self' to '&self'
        // to avoid ***recursive call*** by using $imp::$method(self)
        $imp::$method(&*self)
      }
    }
  };
}
pub use onforward_ref_mut_unop;

/// onforward_ref_binop without Copy derive
#[macro_export]
macro_rules! onforward_ref_binop {
  (impl $imp:ident, $method:ident for $t:ty, $u:ty, $o:ty) => {
    /// impl '$imp'&lt;'$u'$gt; for '$t'
    impl $imp<$u> for $t {
      type Output = $o; // <$o as $imp<$u>>::Output;

      /// '$imp' '$t' '$u'
      #[inline]
      fn $method(self, rhs: $u) -> <$o as $imp<$u>>::Output {
        $imp::$method(&self, &rhs)
      }
    }

    /// impl '$imp'&lt;'&amp;$u'$gt; for '$t'
    impl<'a> $imp<&'a $u> for $t {
      type Output = <$o as $imp<$u>>::Output;

      /// '$imp' '$t' '&amp;$u'
      #[inline]
      fn $method(self, rhs: &'a $u) -> <$o as $imp<$u>>::Output {
        $imp::$method(&self, rhs)
      }
    }

    /// impl '$imp'&lt;'$u'$gt; for '&amp;$t'
    impl<'a> $imp<$u> for &'a $t {
      type Output = <$o as $imp<$u>>::Output;

      /// '$imp' '&amp;$t' '$u'
      #[inline]
      fn $method(self, rhs: $u) -> <$o as $imp<$u>>::Output {
        $imp::$method(self, &rhs)
      }
    }

    crate::ops::onforward_ref_mut_binop!{impl $imp, $method for $t, $u, $o}
  };
}
pub use onforward_ref_binop;

/// onforward_ref_mut_binop without Copy derive
#[macro_export]
macro_rules! onforward_ref_mut_binop {
  (impl $imp:ident, $method:ident for $t:ty, $u:ty, $o:ty) => {
    /// impl '$imp'&lt;'&amp;mut $u'$gt; for '&amp;mut $t'
    impl<'a, 'b> $imp<&'b mut $u> for &'a mut $t {
      type Output = <$o as $imp<$u>>::Output;

      /// '$imp' '&amp;mut $t' '&amp;mut $u'
      #[inline]
      fn $method(self, rhs: &'b mut $u) -> <$o as $imp<$u>>::Output {
        // '&*self' means cast '&mut self' to '&self'
        // to avoid ***recursive call*** by using $imp::$method(self, rhs)
        $imp::$method(&*self, &*rhs)
      }
    }

    /// impl '$imp'&lt;'&amp;mut $u'$gt; for '$t'
    impl<'a> $imp<&'a mut $u> for $t {
      type Output = <$o as $imp<$u>>::Output;

      /// '$imp' '$t' '&amp;mut $u'
      #[inline]
      fn $method(self, rhs: &'a mut $u) -> <$o as $imp<$u>>::Output {
        // '&*self' means cast '&mut self' to '&self'
        // to avoid ***recursive call*** by using $imp::$method(self, rhs)
        $imp::$method(&self, &*rhs)
      }
    }

    /// impl '$imp'&lt;'$u'$gt; for '&amp;mut $t'
    impl<'a> $imp<$u> for &'a mut $t {
      type Output = <$o as $imp<$u>>::Output;

      /// '$imp' '&amp;mut $t' '$u'
      #[inline]
      fn $method(self, rhs: $u) -> <$o as $imp<$u>>::Output {
        // '&*self' means cast '&mut self' to '&self'
        // to avoid ***recursive call*** by using $imp::$method(self, rhs)
        $imp::$method(&*self, &rhs)
      }
    }

    /// impl '$imp'&lt;'&amp;mut $u'$gt; for '&amp;$t'
    impl<'a, 'b> $imp<&'b mut $u> for &'a $t {
      type Output = <$o as $imp<$u>>::Output;

      /// '$imp' '&amp;$t' '&amp;mut $u'
      #[inline]
      fn $method(self, rhs: &'b mut $u) -> <$o as $imp<$u>>::Output {
        // '&*self' means cast '&mut self' to '&self'
        // to avoid ***recursive call*** by using $imp::$method(self, rhs)
        $imp::$method(self, &*rhs)
      }
    }

    /// impl '$imp'&lt;'&amp;$u'$gt; for '&amp;mut $t'
    impl<'a, 'b> $imp<&'b $u> for &'a mut $t {
      type Output = <$o as $imp<$u>>::Output;

      /// '$imp' '&amp;mut $t' '&amp;$u'
      #[inline]
      fn $method(self, rhs: &'b $u) -> <$o as $imp<$u>>::Output {
        // '&*self' means cast '&mut self' to '&self'
        // to avoid ***recursive call*** by using $imp::$method(self, rhs)
        $imp::$method(&*self, rhs)
      }
    }
  };
}
pub use onforward_ref_mut_binop;

/// onforward_ref_op_assign without Copy derive
#[macro_export]
macro_rules! onforward_ref_op_assign {
  (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
    /// impl '$imp'&lt;'$u'&gt; for '$t'
    impl $imp<$u> for $t {
      /// '$imp' '$t' '$u'
      #[inline]
      fn $method(&mut self, rhs: $u) -> () {
        $imp::$method(self, &rhs);
      }
    }

/*
    /// impl '$imp'&lt;'$u'&gt; for &amp;'$t'
    impl<'a> $imp<$u> for &'a $t {
      /// '$imp' &amp;'$t' '$u'
      #[inline]
      fn $method(&mut self, rhs: $u) -> () {
        $imp::$method(*self, &rhs);
      }
    }

    /// impl '$imp'&lt;'&amp;$u'&gt; for &amp;'$t'
    impl<'a, 'b> $imp<&'b $u> for &'a $t {
      /// '$imp' &amp;'$t' '&amp;$u'
      #[inline]
      fn $method(&mut self, rhs: &'b $u) -> () {
        $imp::$method(*self, rhs);
      }
    }
*/

    crate::ops::onforward_ref_mut_op_assign!{impl $imp, $method for $t, $u}
  };
}
pub use onforward_ref_op_assign;

/// onforward_ref_mut_op_assign without Copy derive
#[macro_export]
macro_rules! onforward_ref_mut_op_assign {
  (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
    /// impl '$imp'&lt;'&amp;mut $u'&gt; for '$t'
    impl<'a> $imp<&'a mut $u> for $t {
      /// '$imp' '$t' '&amp;mut $u'
      #[inline]
      fn $method(&mut self, rhs: &'a mut $u) -> () {
        // '&*self' means cast '&mut self' to '&self'
        // to avoid ***recursive call*** by using $imp::$method(self, rhs)
        $imp::$method(self, &*rhs);
      }
    }

/*
    /// impl '$imp'&lt;'&amp;mut $u'&gt; for &amp;'$t'
    impl<'a, 'b> $imp<&'b mut $u> for &'a $t {
      /// '$imp' &amp;'$t' '&amp;mut $u'
      #[inline]
      fn $method(&mut self, rhs: &'b mut $u) -> () {
        // '&*self' means cast '&mut self' to '&self'
        // to avoid ***recursive call*** by using $imp::$method(self, rhs)
        $imp::$method(*self, &*rhs);
      }
    }
*/

    /// impl '$imp'&lt;'$u'&gt; for &amp;mut '$t'
    impl<'a> $imp<$u> for &'a mut $t {
      /// '$imp' &amp;mut '$t' '$u'
      #[inline]
      fn $method(&mut self, rhs: $u) -> () {
        $imp::$method(*self, &rhs);
      }
    }

    /// impl '$imp'&lt;'&amp;$u'&gt; for &amp;mut '$t'
    impl<'a, 'b> $imp<&'b $u> for &'a mut $t {
      /// '$imp' &amp;mut '$t' '&amp;$u'
      #[inline]
      fn $method(&mut self, rhs: &'b $u) -> () {
        $imp::$method(*self, rhs);
      }
    }

    /// impl '$imp'&lt;'&amp;mut $u'&gt; for &amp;mut '$t'
    impl<'a, 'b> $imp<&'b mut $u> for &'a mut $t {
      /// '$imp' &amp;mut '$t' '&amp;mut $u'
      #[inline]
      fn $method(&mut self, rhs: &'b mut $u) -> () {
        // '&*self' means cast '&mut self' to '&self'
        // to avoid ***recursive call*** by using $imp::$method(self, rhs)
        $imp::$method(*self, &*rhs);
      }
    }
  };
}
pub use onforward_ref_mut_op_assign;
