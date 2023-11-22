Feature: Check if number too big

  Scenario: Pass number bigger then guessed
    Given number for guess is 3
    Then game is started
    Then it prints welcome message

    Then it asks number
    When we guess 4
    Then it prints we asked 4
    Then it prints number too big

    Then it asks number
    When we guess 3
    Then it prints we asked 3

    Then it prints we win!

