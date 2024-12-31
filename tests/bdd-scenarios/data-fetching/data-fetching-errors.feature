Feature: Fetching Data Errors

Scenario: Failed to fetch data successfully
    Given: The request made to the endpoint is valid
    When: The request is made
    Then: The response declares the request was unsuccessful 
    And: FetchError is returned

Scenario: Failed to lock mutex and therefore update live data store
    Given: Given the request returned successfully
    When: The data is attempted to be written to the live data store (i.e. the struct)
    Then: The mutex cannot be accessed safely 
    And: a MutexError is returned