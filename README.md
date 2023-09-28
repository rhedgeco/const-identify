# Const Identify

Generates unique ids that can be used at compile time.

Also comes with array and slice types to manage collections of ids.

## Usage

```rust
#[derive(ConstIdentify)]
pub struct Struct1;

#[derive(ConstIdentify)]
pub struct Struct2;

#[derive(ConstIdentify)]
pub struct Struct3;

// create an ordered id array
// these are arrays of ids that are pre sorted
const ORDERED: OrderedIdArray<2> = ordered_ids![Struct1, Struct1, Struct2];

// create a unique id array
// these are arrays of ids that are guarunteed to be unique
const UNIQUE: UniqueIdArray<3> = match unique_ids![Struct1, Struct2, Struct3] {
    Some(unique) => unique,
    // compile time panics can be bubbled up preventing compilation if uniqueness is not held
    None => panic!("Duplicate ids detected"),
}

// arrays may be compared at compile time using the provided methods
const COMPARE: std::cmp::Ordering = ORDERED.const_cmp_unique(&UNIQUE);

// arrays provide raw access to the underlying slices
const SORTED_SLICE: &'static [ConstId] = ORDERED.as_raw_slice();

// arrays may be converted to special slice types that carry the ordering/uniqueness guaruntee
const ORDERED_SLICE: OrderedIdSlice<'static> = OrderedIdSlice::from_arr(&ORDERED);
const UNIQUE_SLICE: UniqueIdSlice<'static> = UniqueIdSlice::from_arr(&UNIQUE);
```

### [MIT LICENSE](LICENSE.md)
