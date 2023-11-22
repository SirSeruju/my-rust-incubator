Feature: Check if number too big

  Scenario: Pass number bigger then guessed
    Given number for guess is 3
    Then game is started
    Then it prints welcome message

    Then it asks number
    When we guess NAN

    Then it asks number
    When we guess 3

    # TODO: FIX IT
    # Why "Please input your guess." is doubled ???
    Then it asks number

    Then it prints we asked 3

    Then it prints we win!
