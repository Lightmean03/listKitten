# listKitten 


## Prerequisites

Before you begin, ensure that you have the following prerequisites installed on your system:

- **Rust Programming Language:** This application is developed in Rust. Inorder to compile make sure you have Rust installed on your machine. You can download Rust from [Rust's official website](https://www.rust-lang.org/).

## Installation

To compile and run the Rust List Combination Application, follow the steps below:

1. **Clone the Repository:**
    ```bash
    $ git clone https://github.com/Lightmean03/listKitten.git
    $ cd listKitten
    ```

2. **Compile the Application:**
    ```bash
    $ cargo  build -r
    ```

3. **Run the Application:**
   ```bash
    $ target/release/
    $ ./listKitten -m mode -f file1.txt -F file2.txt -o output.txt
    ```

## Usage


listKitten accepts the following command-line options:

- **Mode (`-m` or `--mode`):** Specify the mode of operation. Choose from the following options:
  - `left`: Combine the first list with the second list.
  - `right`: Combine the second list with the first list.
  - `all_space`: Combine the lists with a space between each word.
  - `all`: Combine the lists in both directions.
  - `pattern`: Apply a specific pattern to manipulate the lists.

- **Pattern (`-p` or `--pattern`):** Specify a pattern when using the `pattern` mode. The pattern can include:
  - `N`: Numbers 0-9.
  - `Z`: Alphabet.
  - `_`: Space.

- **File Path 1 (`-f` or `--file1`):** Path to the first input file.

- **File Path 2 (`-F` or `--file2`):** Path to the second input file.

- **Output File (`-o` or `--output`):** Path to the output file.

## Example Commaneds 

Here are some examples of how to use the Rust List Combination Application with different modes:


1. Combine lists with spaces:

    ```bash
    $ ./listKitten -m all_space -f list1.txt -F list2.txt -o output.txt
    ```

2. Apply a pattern to manipulate lists:

    ```bash
    $ ./listKitten -m pattern -p N_Z -f list1.txt -F list2.txt -o output.txt
    ```
### Example: Combine Lists in Both Directions

Consider the scenario where we have two input lists, `list1.txt` and `list2.txt`. We want to combine these lists in both directions, creating a new output list named `output.txt`.

```bash
$ cat list1.txt
red
blue
green
brown
$ cat list2.txt
john
jake
jac
```
```bash
$ ./listKitten -m all -f list1.txt -F list2.txt -o output.txt
```

```bash
$ cat output.txt
redjohn
redjake
redjack
bluejohn
bluejake
bluejack
greenjohn
greenjake
greenjack
brownjohn
brownjake
brownjack
johnred
johnblue
johngreen
johnbrown
jakered
jakeblue
jakegreen
jakebrown
jackred
jackblue
jackgreen
jackbrown
```



