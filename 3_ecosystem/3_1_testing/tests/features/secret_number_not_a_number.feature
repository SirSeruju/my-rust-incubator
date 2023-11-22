Feature: Check if secret number passed to args not a number

  Scenario:
    Given number for guess is NAN
    Then game is started
    Then it paniced
