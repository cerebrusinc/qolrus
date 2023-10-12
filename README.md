<p align="center">
    <img src="https://drive.google.com/uc?id=1K58mXW9udC7_TM7L4JujAZbLJNFa-sSb" alt="qolrus logo" width="250" height="250" />
</p>

# qolrus

A suite of random but useful functions that are aimed at giving you 'piece of cake' level comfortability.

This is a port of the following packages:

- [qol • js/ts](https://www.npmjs.com/package/@cerebrusinc/qol)
- [qolpy • py](https://pypi.org/project/qolpy)

Any suggestions or fixes needed can be submitted [here](https://github.com/cerebrusinc/qolrus/issues).

# Functions

<details>
<summary><strong>random_color</strong></summary>

Generate a random colour.

**Example**

```rs
use qolpy::{random_colour, ColourType};

fn main() {
    let colour: String = random_colour(ColourType::HEX);
    println!("The colour is: '{}'", colour)
    // The colour is: '#f7f7f7'
}
```

**Availabe (ENUM) colour types**

- HEX
- RBG
- CMYK
- HSV
- HSL
</details>
