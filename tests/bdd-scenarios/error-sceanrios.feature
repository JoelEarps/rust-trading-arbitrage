Feature: Handling of the arbitrage loop

Scenario 1: Arbitrage opportunities cause infinite loop for source vertex
    Given: The map has a recursive loop which will result in an infinite arbitrage loop
    When: The arbitrage loop is reconstructed
    Then: It can break the loop and handle the erroneous scenario.

Scenario 2: Arbitrage opportunities 
    Given: The map has a arbitrage opportnities
    When: The arbitrage loop is reconstructed
    Then: The opportunities are pretty printed and calculated for a user input