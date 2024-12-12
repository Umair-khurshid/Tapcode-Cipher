# Tapcode Cipher  [![Rust](https://github.com/Umair-khurshid/Tapcode-Cipher/actions/workflows/rust.yml/badge.svg)](https://github.com/Umair-khurshid/Tapcode-Cipher/actions/workflows/rust.yml) [![rust-clippy analyze](https://github.com/Umair-khurshid/Tapcode-Cipher/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/Umair-khurshid/Tapcode-Cipher/actions/workflows/rust-clippy.yml)
Tapcode Cipher is a simple command-line application written in Rust that implements the Tapcode or Prison cipher, which is a system of encoding and decoding messages using a 5x5 grid of letters. Each letter is represented by a series of "taps," where the number of taps corresponds to the position of the letter in the grid. 

## Features:
- **Encode a message**: Converts a message into Tapcode using the current grid.
- **Decode Tapcode**: Converts Tapcode back into a message using the current grid.
- **Custom Grid**: Users can create and customize their own 5x5 grid for encoding/decoding.
- **Save Grid**: Save the current grid to a file for future use.
- **Load Grid**: Load a saved grid from a file.
.
## Usage Instructions:
Run the program using `cargo run` to start the interactive command-line interface.
Choose an option from the menu:

![menu](https://github.com/user-attachments/assets/fb071edf-fecb-4e01-a31d-d09bda72c593)

### Example:
**Encoding**:

![encode](https://github.com/user-attachments/assets/758a139f-cfc8-4864-aa60-86d475c6951c)

**Decoding**

![decode](https://github.com/user-attachments/assets/43ce3dcf-96bb-4e12-a6d5-0ab469c004ff)
