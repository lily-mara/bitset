pub use super::{ BitSet, DEFAULT_SIZE };

describe! bit_set {
    describe! default_size {
        before_each {
            let mut set = BitSet::new();
        }

        it "has a default size" {
            assert!(set.capacity() == DEFAULT_SIZE);
        }

        it "contains only the number inserted" {
            let index = 50;
            set.insert(index);

            for i in 0..index {
                assert!(!set.contains(i));
            }

            for i in (index + 1)..DEFAULT_SIZE {
                assert!(!set.contains(i));
            }

            assert!(set.contains(index));
        }

        it "doesnt contain any in range" {
            let cap = DEFAULT_SIZE;
            for i in 0..cap {
                assert!(!set.contains(i));
            }
        }
    }

    describe! with_capacity_10 {
        before_each {
            let mut set = BitSet::new_with_capacity(10);
        }

        it "doesnt contain 1 until 1 is inserted" {
            assert!(!set.contains(1));
            set.insert(1);
            assert!(set.contains(1));
        }

        it "doesnt contain any in capacity" {
            for i in 0..10 {
                assert!(!set.contains(i));
            }
        }
    }
}
