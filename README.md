
# Ethereum Price Predictions Using Monte Carlo Simulation

This Rust-based application employs the Monte Carlo simulation methodology to predict Ethereum's future prices based on its past three years of historical price data from Coingecko.

## Table of Contents

- [Technical Overview](#technical-overview)
- [Dependencies](#dependencies)
- [Setup and Execution](#setup-and-execution)
- [Code Structure and Explanation](#code-structure-and-explanation)
- [Output](#output)
- [Contributing](#contributing)

## Technical Overview

The Monte Carlo method involves generating a large number of random samples to determine an expected outcome. In the context of this application, the method is used to simulate potential Ethereum price trajectories based on its historical daily returns.

## Dependencies

- `serde_derive`: For deriving the Deserialize trait, which is used for deserializing the fetched data.
- `reqwest`: A modern Rust HTTP client used to fetch Ethereum's historical data.
- `serde` and `serde_json`: For serializing and deserializing JSON data.
- `rand`: For generating random numbers, especially when simulating daily returns.
- `plotters`: For plotting the simulated data on a chart.

## Setup and Execution

1. **Prerequisites**: Ensure you have Rust and Cargo installed on your machine.
2. **Clone the Repository**:
   ```bash
   git clone https://github.com/codeesura/ethereum-price-predictions-using-monte-carlo-simulation
   ```
3. **Navigate to the Project Directory**:
   ```bash
   cd ethereum-price-predictions-using-monte-carlo-simulation
   ```
4. **Run the Application**:
   ```bash
   cargo run
   ```

## Code Structure and Explanation

- **Structs and Externals**:
  - `EthereumData` struct: Represents the response format from Coingecko, containing Ethereum's historical prices.
  
- **Main Function**:
  - **Fetching Data**: The code initiates an asynchronous HTTP GET request to fetch the last three years of Ethereum's daily price data.
  - **Calculating Daily Returns**: For each consecutive day, the daily return is calculated using the formula:

    ![Formula](https://quicklatex.com/cache3/3f/ql_31f4c2e362b1696c3b53dc7e00a24e3f_l3.png)

  - **Monte Carlo Simulation**: The program simulates the price of Ethereum for the next year (365 days) by randomly selecting from the calculated daily returns and applying them to the last known price.
  - **Plotting**: The program uses the `plotters` crate to plot the simulated prices. The median of all simulations for each day is highlighted.

## Output

After successful execution, an image named `ethereum_predictions.png` will be generated in the root directory. This image displays the Ethereum price predictions with the median price trajectory highlighted.

## Contributing

For any suggestions, bug reports, or improvements, please open an issue or submit a pull request. Ensure that your code adheres to the Rust conventions and is well-documented.
