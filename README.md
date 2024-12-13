# **MY_TORCH Documentation**

## **1. Introduction**

### Project Overview
MY_TORCH is a project designed to develop two key binaries:
1. **Neural Network Generator**: Generates neural networks based on configuration files.
2. **Chessboard Analyzer**: Analyzes chessboard states using neural networks in training or evaluation modes.

The project combines cryptography and artificial intelligence to simulate optimal strategies for chess games.

### Objectives and Deliverables
- Deliver two binaries: `my_torch_generator` and `my_torch_analyzer`.
- Implement a supervised learning-based AI.

### Key Features
- No external machine learning libraries (e.g., PyTorch, TensorFlow).
- Flexible configuration for neural network generation.
- FEN (Forsyth–Edwards Notation) parsing for chessboard inputs.

---

## **2. Getting Started**

### Prerequisites
- Rust (latest stable version)
- `cargo` (Rust package manager)
- Additional tools:
  - A text editor or IDE (e.g., VSCode, IntelliJ IDEA with Rust plugin)

### Installation and Compilation
#### Cloning the Repository:
```bash
git clone https://github.com/Thomaltarix/Neural-Network.git
cd Neural-Network
```

#### Building Binaries:
```bash
make
```
This will generate the `my_torch_generator` and `my_torch_analyzer` binaries.

### Running the Programs

To see each program's usage, run the following commands:
```bash
./my_torch_generator --help
./my_torch_analyzer --help
```

#### Neural Network Generator:
```bash
./my_torch_generator <config_file> <number_of_networks>
```
Example:
```bash
./my_torch_generator basic_network.conf 3
```

#### Chessboard Analyzer:
**Training Mode:**
```bash
./my_torch_analyzer --train [--save <save_file>] <network_file> <chessboard_file> 
```
**Prediction Mode:**
```bash
./my_torch_analyzer --predict <network_file> <chessboard_file>
```

---

## **3. Technical Design**

### Architecture
- **Generator:**
  - Parses a configuration file to define hyperparameters.
  - Generates one or more neural networks as `.nn` files.
- **Analyzer:**
  - Accepts `.nn` files for training or prediction.
  - Processes FEN-encoded chessboards for analysis.

### Neural Network Design
- **Configuration Format:**
  - Specify layers, activation functions, and hyperparameters.
  - Example:
    ```
    input_size: 64
    layers: [128, 64, 32]
    activation: relu
    learning_rate: 0.01
    ```

- **Output:**
  - Chessboard state predictions: `Checkmate`, `Check`, `Stalemate`, `Nothing`.

### Chessboard Representation
- **FEN Notation:**
  - Encodes chess positions in a single line.
  - Example: `rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1`
- **Parsing:**
  - Extract board state, active color, and castling availability.

---

## **4. Development Guidelines**

### Coding Standards
- Follow Rust conventions.
- Use descriptive variable and function names.

### Error Handling
- Exit codes:
  - `0`: Success
  - `84`: Failure

### Testing
- Write unit tests for all modules using Rust’s `cargo test`.
- Example test case:
```rust
#[test]
fn test_activation_function() {
    assert_eq!(relu(-1.0), 0.0);
    assert_eq!(relu(2.0), 2.0);
}
```

### Debugging Tips
- Use `println!` for quick debugging.
- Use Rust’s `debug_assert!` to validate assumptions during development.

---

## **5. Machine Learning**

### Training Process
- **Input:**
  - FEN positions with expected outcomes.
- **Output:**
  - Predicted game states.

### Avoiding Overfitting
- Techniques:
  - Cross-validation
  - Regularization (e.g., weight decay)

### Hyperparameter Optimization
- Adjust learning rate and initialization values.
- Automate optimization using grid search or random search.

---

## **6. Optimizations**

### Optimization Techniques
- Multithreading for parallel data processing (not implemented yet).
- Learning rate scheduling for faster convergence.

## **7. Usage Examples**

### Generating a Neural Network
```bash
./my_torch_generator network.conf 5
```

### Training with a Dataset
```bashi
./my_torch_analyzer --train --save trained_network.nn my_torch_network.nn chessboards.txt 
```

### Making Predictions
```bash
./my_torch_analyzer --predict trained_network.nn chessboards.txt
```

---

## **8. FAQ and Troubleshooting**

### Common Issues
1. **Compilation Errors:** Ensure Rust and dependencies are correctly installed.
2. **Invalid FEN Input:** Validate FEN strings before processing.

### Debugging Steps
- Run tests using `cargo test`.

---

## **9. Future Enhancements**
- Full chess AI implementation.
- Support for additional input formats.
- Enhanced visualization for predictions.

---

## **10. Appendices**

### Glossary of Terms
- **FEN:** A standard notation for describing chess positions.
- **Hyperparameter:** Configurable variables used to train neural networks.

### References
- [FEN Notation](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)
- [Hyperparameter Optimization](https://en.wikipedia.org/wiki/Hyperparameter_optimization)
