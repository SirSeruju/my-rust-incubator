Feature: Check if number too small

  Scenario: Pass number smaller then guessed
    Given number for guess is 3
    Then game is started
    Then it prints welcome message

    Then it asks number
    When we guess 2
    Then it prints we asked 2
    Then it prints number too small

    Then it asks number
    When we guess 3
    Then it prints we asked 3

    Then it prints we win!

