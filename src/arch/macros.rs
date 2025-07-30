macro_rules! bit {
    ($bit:expr) => {
        (1 << $bit)
    };
}

macro_rules! impl_field {
    ($setter:ident, $getter:ident, $field:ident[$idx:expr]) => {
        /// Set the value of the field
        pub const fn $setter(&mut self, value: usize) {
            self.$field[$idx] = value;
        }

        /// Get the value of the field
        pub const fn $getter(&self) -> usize {
            self.$field[$idx]
        }
    };
    ($setter:ident, $getter:ident, $field:ident) => {
        /// Set the value of the field
        pub const fn $setter(&mut self, value: usize) {
            self.$field = value;
        }

        /// Get the value of the field
        pub const fn $getter(&self) -> usize {
            self.$field
        }
    };
}
