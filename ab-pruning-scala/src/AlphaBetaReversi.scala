import scala.collection.mutable
import scala.collection.mutable.ListBuffer
import scala.io.AnsiColor
import scala.util.control.Breaks.{break, breakable}
import scala.io.AnsiColor.*

object main extends App {

  var simulations = 0

  private val board: ListBuffer[ListBuffer[String]] = mutable.ListBuffer(
    ListBuffer(" ", " ", " ", " ", " ", " ", " ", " "),
    ListBuffer(" ", " ", " ", " ", " ", " ", " ", " "),
    ListBuffer(" ", " ", " ", " ", " ", " ", " ", " "),
    ListBuffer(" ", " ", " ", "O", "X", " ", " ", " "),
    ListBuffer(" ", " ", " ", "X", "O", " ", " ", " "),
    ListBuffer(" ", " ", " ", " ", " ", " ", " ", " "),
    ListBuffer(" ", " ", " ", " ", " ", " ", " ", " "),
    ListBuffer(" ", " ", " ", " ", " ", " ", " ", " "),
  )

  //  private val board = mutable.HashMap(
  //    1 -> " ", 2 -> " ", 3 -> " ", 4 -> " ", 5 -> " ", 6 -> " ", 7 -> " ", 8 -> " ",
  //    9 -> " ", 10 -> " ", 11 -> " ", 12 -> " ", 13 -> " ", 14 -> " ", 15 -> " ", 16 -> " ",
  //    17 -> " ", 18 -> " ", 19 -> " ", 20 -> " ", 21 -> " ", 22 -> " ", 23 -> " ", 24 -> " ",
  //    25 -> " ", 26 -> " ", 27 -> " ", 28 -> " ", 29 -> " ", 30 -> " ", 31 -> " ", 32 -> " ",
  //    33 -> " ", 34 -> " ", 35 -> " ", 36 -> " ", 37 -> " ", 38 -> " ", 39 -> " ", 40 -> " ",
  //    41 -> " ", 42 -> " ", 43 -> " ", 44 -> " ", 45 -> " ", 46 -> " ", 47 -> " ", 48 -> " ",
  //    49 -> " ", 50 -> " ", 51 -> " ", 52 -> " ", 53 -> " ", 54 -> " ", 55 -> " ", 56 -> " ",
  //    57 -> " ", 58 -> " ", 59 -> " ", 60 -> " ", 61 -> " ", 62 -> " ", 63 -> " ", 64 -> " "
  //  )

  //  def coordinationsBoard(): Unit = {
  //    println()
  //    println(" 1  | 2  | 3  | 4  | 5  | 6  | 7  | 8  ")
  //    println("----+----+----+----+----+----+----+----")
  //    println(" 9  | 10 | 11 | 12 | 13 | 14 | 15 | 16 ")
  //    println("----+----+----+----+----+----+----+----")
  //    println(" 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 ")
  //    println("----+----+----+----+----+----+----+----")
  //    println(" 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 ")
  //    println("----+----+----+----+----+----+----+----")
  //    println(" 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 ")
  //    println("----+----+----+----+----+----+----+----")
  //    println(" 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 ")
  //    println("----+----+----+----+----+----+----+----")
  //    println(" 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 ")
  //    println("----+----+----+----+----+----+----+----")
  //    println(" 57 | 58 | 59 | 60 | 61 | 62 | 63 | 64 ")
  //  }

  //  def currentBoard(): Unit = {
  //    println()
  //    println(" " +board(1)+ "  | " +board(2)+ "  | " +board(3)+ "  | " +board(4)+ "  | " +board(5)+ "  | " +board(6)+ "  | " +board(7)+ "  | " +board(8))
  //    println("----+----+----+----+----+----+----+----")
  //    println(" " +board(9)+ "  | " +board(10)+ "  | " +board(11)+ "  | " +board(12)+ "  | " +board(13)+ "  | " +board(14)+ "  | " +board(15)+ "  | " +board(16))
  //    println("----+----+----+----+----+----+----+----")
  //    println(" " +board(17)+ "  | " +board(18)+ "  | " +board(19)+ "  | " +board(20)+ "  | " +board(21)+ "  | " +board(22)+ "  | " +board(23)+ "  | " +board(24))
  //    println("----+----+----+----+----+----+----+----")
  //    println(" " +board(25)+ "  | " +board(26)+ "  | " +board(27)+ "  | " +board(28)+ "  | " +board(29)+ "  | " +board(30)+ "  | " +board(31)+ "  | " +board(32))
  //    println("----+----+----+----+----+----+----+----")
  //    println(" " +board(33)+ "  | " +board(34)+ "  | " +board(35)+ "  | " +board(36)+ "  | " +board(37)+ "  | " +board(38)+ "  | " +board(39)+ "  | " +board(40))
  //    println("----+----+----+----+----+----+----+----")
  //    println(" " +board(41)+ "  | " +board(42)+ "  | " +board(43)+ "  | " +board(44)+ "  | " +board(45)+ "  | " +board(46)+ "  | " +board(47)+ "  | " +board(48))
  //    println("----+----+----+----+----+----+----+----")
  //    println(" " +board(49)+ "  | " +board(50)+ "  | " +board(51)+ "  | " +board(52)+ "  | " +board(53)+ "  | " +board(54)+ "  | " +board(55)+ "  | " +board(56))
  //    println("----+----+----+----+----+----+----+----")
  //    println(" " +board(57)+ "  | " +board(58)+ "  | " +board(59)+ "  | " +board(60)+ "  | " +board(61)+ "  | " +board(62)+ "  | " +board(63)+ "  | " +board(64))
  //  }

  //  def coordinationsBoard(): Unit = {
  //    println()
  //    println("   | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 ")
  //    println("0  |   |   |   |   |   |   |   |   ")
  //    println("   +---+---+---+---+---+---+---+---")
  //    println("1    |    |    |    |    |    |    |    ")
  //    println("----+----+----+----+----+----+----+----")
  //    println("2    |    |    |    |    |    |    |    ")
  //    println("----+----+----+----+----+----+----+----")
  //    println("3    |    |    |    |    |    |    |    ")
  //    println("----+----+----+----+----+----+----+----")
  //    println("4    |    |    |    |    |    |    |    ")
  //    println("----+----+----+----+----+----+----+----")
  //    println("5    |    |    |    |    |    |    |    ")
  //    println("----+----+----+----+----+----+----+----")
  //    println("6    |    |    |    |    |    |    |    ")
  //    println("----+----+----+----+----+----+----+----")
  //    println("7    |    |    |    |    |    |    |    ")
  //  }

  //  def coordinationsBoard(): Unit = {
  //    println()
  //    println("     0   1   2   3   4   5   6   7 ")
  //    println("0  |   |   |   |   |   |   |   |   ")
  //    println("   +---+---+---+---+---+---+---+---")
  //    println("1  |   |   |   |   |   |   |   |   ")
  //    println("   +---+---+---+---+---+---+---+---")
  //    println("2  |   |   |   |   |   |   |   |   ")
  //    println("   +---+---+---+---+---+---+---+---")
  //    println("3  |   |   |   |   |   |   |   |   ")
  //    println("   +---+---+---+---+---+---+---+---")
  //    println("4  |   |   |   |   |   |   |   |   ")
  //    println("   +---+---+---+---+---+---+---+---")
  //    println("5  |   |   |   |   |   |   |   |   ")
  //    println("   +---+---+---+---+---+---+---+---")
  //    println("6  |   |   |   |   |   |   |   |   ")
  //    println("   +---+---+---+---+---+---+---+---")
  //    println("7  |   |   |   |   |   |   |   |   ")
  //  }

  def currentBoard(): Unit = {
    println()
    println("     0   1   2   3   4   5   6   7 ")
    println(s"0  | ${BOLD}${selectColor(0,0)}${board(0)(0)}${RESET} | ${BOLD}${selectColor(0,1)}${board(0)(1)}${RESET} | ${BOLD}${selectColor(0,2)}${board(0)(2)}${RESET} | ${BOLD}${selectColor(0,3)}${board(0)(3)}${RESET} | ${BOLD}${selectColor(0,4)}${board(0)(4)}${RESET} | ${BOLD}${selectColor(0,5)}${board(0)(5)}${RESET} | ${BOLD}${selectColor(0,6)}${board(0)(6)}${RESET} | ${BOLD}${selectColor(0,7)}${board(0)(7)}${RESET} ")
    println("   +---+---+---+---+---+---+---+---")
    println(s"1  | ${BOLD}${selectColor(1,0)}${board(1)(0)}${RESET} | ${BOLD}${selectColor(1,1)}${board(1)(1)}${RESET} | ${BOLD}${selectColor(1,2)}${board(1)(2)}${RESET} | ${BOLD}${selectColor(1,3)}${board(1)(3)}${RESET} | ${BOLD}${selectColor(1,4)}${board(1)(4)}${RESET} | ${BOLD}${selectColor(1,5)}${board(1)(5)}${RESET} | ${BOLD}${selectColor(1,6)}${board(1)(6)}${RESET} | ${BOLD}${selectColor(1,7)}${board(1)(7)}${RESET} ")
    println("   +---+---+---+---+---+---+---+---")
    println(s"2  | ${BOLD}${selectColor(2,0)}${board(2)(0)}${RESET} | ${BOLD}${selectColor(2,1)}${board(2)(1)}${RESET} | ${BOLD}${selectColor(2,2)}${board(2)(2)}${RESET} | ${BOLD}${selectColor(2,3)}${board(2)(3)}${RESET} | ${BOLD}${selectColor(2,4)}${board(2)(4)}${RESET} | ${BOLD}${selectColor(2,5)}${board(2)(5)}${RESET} | ${BOLD}${selectColor(2,6)}${board(2)(6)}${RESET} | ${BOLD}${selectColor(2,7)}${board(2)(7)}${RESET} ")
    println("   +---+---+---+---+---+---+---+---")
    println(s"3  | ${BOLD}${selectColor(3,0)}${board(3)(0)}${RESET} | ${BOLD}${selectColor(3,1)}${board(3)(1)}${RESET} | ${BOLD}${selectColor(3,2)}${board(3)(2)}${RESET} | ${BOLD}${selectColor(3,3)}${board(3)(3)}${RESET} | ${BOLD}${selectColor(3,4)}${board(3)(4)}${RESET} | ${BOLD}${selectColor(3,5)}${board(3)(5)}${RESET} | ${BOLD}${selectColor(3,6)}${board(3)(6)}${RESET} | ${BOLD}${selectColor(3,7)}${board(3)(7)}${RESET} ")
    println("   +---+---+---+---+---+---+---+---")
    println(s"4  | ${BOLD}${selectColor(4,0)}${board(4)(0)}${RESET} | ${BOLD}${selectColor(4,1)}${board(4)(1)}${RESET} | ${BOLD}${selectColor(4,2)}${board(4)(2)}${RESET} | ${BOLD}${selectColor(4,3)}${board(4)(3)}${RESET} | ${BOLD}${selectColor(4,4)}${board(4)(4)}${RESET} | ${BOLD}${selectColor(4,5)}${board(4)(5)}${RESET} | ${BOLD}${selectColor(4,6)}${board(4)(6)}${RESET} | ${BOLD}${selectColor(4,7)}${board(4)(7)}${RESET} ")
    println("   +---+---+---+---+---+---+---+---")
    println(s"5  | ${BOLD}${selectColor(5,0)}${board(5)(0)}${RESET} | ${BOLD}${selectColor(5,1)}${board(5)(1)}${RESET} | ${BOLD}${selectColor(5,2)}${board(5)(2)}${RESET} | ${BOLD}${selectColor(5,3)}${board(5)(3)}${RESET} | ${BOLD}${selectColor(5,4)}${board(5)(4)}${RESET} | ${BOLD}${selectColor(5,5)}${board(5)(5)}${RESET} | ${BOLD}${selectColor(5,6)}${board(5)(6)}${RESET} | ${BOLD}${selectColor(5,7)}${board(5)(7)}${RESET} ")
    println("   +---+---+---+---+---+---+---+---")
    println(s"6  | ${BOLD}${selectColor(6,0)}${board(6)(0)}${RESET} | ${BOLD}${selectColor(6,1)}${board(6)(1)}${RESET} | ${BOLD}${selectColor(6,2)}${board(6)(2)}${RESET} | ${BOLD}${selectColor(6,3)}${board(6)(3)}${RESET} | ${BOLD}${selectColor(6,4)}${board(6)(4)}${RESET} | ${BOLD}${selectColor(6,5)}${board(6)(5)}${RESET} | ${BOLD}${selectColor(6,6)}${board(6)(6)}${RESET} | ${BOLD}${selectColor(6,7)}${board(6)(7)}${RESET} ")
    println("   +---+---+---+---+---+---+---+---")
    println(s"7  | ${BOLD}${selectColor(7,0)}${board(7)(0)}${RESET} | ${BOLD}${selectColor(7,1)}${board(7)(1)}${RESET} | ${BOLD}${selectColor(7,2)}${board(7)(2)}${RESET} | ${BOLD}${selectColor(7,3)}${board(7)(3)}${RESET} | ${BOLD}${selectColor(7,4)}${board(7)(4)}${RESET} | ${BOLD}${selectColor(7,5)}${board(7)(5)}${RESET} | ${BOLD}${selectColor(7,6)}${board(7)(6)}${RESET} | ${BOLD}${selectColor(7,7)}${board(7)(7)}${RESET} ")
  }

  def selectColor(y: Int, x: Int): String = {
    if (board(y)(x) == "X") RED
    else CYAN
  }

  def parseInput(input: String): Array[String] = {
    input.trim.split(" ")
  }

  def helpCommand(): Unit = {
    println("\nAVAILABLE COMMANDS:")
    println("- 'help'")
    println("- 'quit'")
    println("- select (x, y) coordinates to board spots below")
    currentBoard()
    println()
  }

  def isCoordMove(move: String): Boolean = { // check coordinate moves
    val moves = List.range(0, 8).map(_.toString)
    moves.contains(move)
  }

  def makeMove(move: (Int, Int), player: String): Unit = {
    val x = move._1
    val y = move._2
    board(y)(x) = player
    val opp = if player == "X" then "O" else "X"
    // check left
    if checkFlip(x - 1, y, -1, 0, player, opp) then flipPieces(x - 1, y, -1, 0, player, opp)
    // check right
    if checkFlip(x + 1, y, 1, 0, player, opp) then flipPieces(x + 1, y, 1, 0, player, opp)
    // check down
    if checkFlip(x, y - 1, 0, -1, player, opp) then flipPieces(x, y - 1, 0, -1, player, opp)
    // check up
    if checkFlip(x, y + 1, 0, 1, player, opp) then flipPieces(x, y + 1, 0, 1, player, opp)
    // check down-left
    if checkFlip(x - 1, y - 1, -1, -1, player, opp) then flipPieces(x - 1, y - 1, -1, -1, player, opp)
    // check down-right
    if checkFlip(x + 1, y - 1, 1, -1, player, opp) then flipPieces(x + 1, y - 1, 1, -1, player, opp)
    // check up-left
    if checkFlip(x - 1, y + 1, -1, 1, player, opp) then flipPieces(x - 1, y + 1, -1, 1, player, opp)
    // check up-right
    if checkFlip(x + 1, y + 1, 1, 1, player, opp) then flipPieces(x + 1, y + 1, 1, 1, player, opp)
  }

  def flipPieces(x: Int, y: Int, nx: Int, ny: Int, player: String, opp: String): Unit = {
    var tempX = x
    var tempY = y
    while (board(tempY)(tempX) == opp) {
      board(tempY)(tempX) = player
      tempX += nx
      tempY += ny
    }
  }

  def checkFlip(x: Int, y:Int, nx: Int, ny:Int , player: String, opp: String): Boolean = {
    if ((x < 0) || (x >= 8) || (y < 0) || (y >= 8)) return false
    if (board(y)(x) == opp) {
      var tempX = x
      var tempY = y
      while ((x >= 0) && (x < 8) && (y >= 0) && (y < 8)) {
        tempX = tempX + nx
        tempY = tempY + ny
        if ((tempX < 0) || (tempX >= 8) || (tempY < 0) || (tempY >= 8)) return false
        if board(tempY)(tempX) == " " then return false
        if board(tempY)(tempX) == player then return true
      }
    }
    false
  }

  def isValidMove(move: (Int, Int), player: String): Boolean = {
    // reversi valid move
    val x = move._1
    val y = move._2
    if ((x < 0) || (x > 8) || (y < 0) || (y > 8)) return false
    val opp = if player == "X" then "O" else "X"
    if (board(y)(x) == " ") {
      // check left
      if checkFlip(x-1, y, -1, 0, player, opp) then return true
      // check right
      if checkFlip(x+1, y, 1, 0, player, opp) then return true
      // check down
      if checkFlip(x, y-1, 0, -1, player, opp) then return true
      // check up
      if checkFlip(x, y+1, 0, 1, player, opp) then return true
      // check down-left
      if checkFlip(x-1, y-1, -1, -1, player, opp) then return true
      // check down-right
      if checkFlip(x+1, y-1, 1, -1, player, opp) then return true
      // check up-left
      if checkFlip(x-1, y+1, -1, 1, player, opp) then return true
      // check up-right
      if checkFlip(x+1, y+1, 1, 1, player, opp) then return true

      false
    }
    else false
  }

  def isValidCommand(commands: Array[String]): Boolean = {
    if (commands.length == 1) {
      val command = commands.head.toLowerCase()
      if (command == "help" || command == "quit") {
        return true
      }
    }
    else if (commands.length == 2) {
      if (isCoordMove(commands(0)) && isCoordMove(commands(1))) {
        return true
      }
    }
    false
  }

  def calculateMoveMinimax(depth: Int, isMaximizer: Boolean, playerXO: String, aiXO: String): (Int, Int) = { // return coordinates
    var bestScore = Integer.MIN_VALUE
    var bestMove = (-1,-1)
    for (y <- 0 to 7) {
      for (x <- 0 to 7) {
        if (board(y)(x) == " ") {
          board(y)(x) = aiXO
          val score = minimax(board.clone(), 0, false, aiXO, playerXO)
          board(y)(x) = " "
          if (score > bestScore) {
            bestScore = score
            bestMove = (y, x)
          }
        }
      }
    }
    println("end game simulations went thru: " + simulations)
    simulations = 0
    bestMove
  }

  def evaluateStateScore(copiedBoard: ListBuffer[ListBuffer[String]], playerXO: String, aiXO: String): (Int, Int) = { // X's score, O's score
    var playerScore = 0
    var aiScore = 0
    //    board.map(y => y.map(x => {
    //      if (x == playerXO) playerScore += 1 else if (x == aiXO) aiScore += 1
    //    }))
    for (y <- 0 to 7) {
      for (x <- 0 to 7) {
        val state = copiedBoard(y)(x)
        if (state == playerXO) {
          playerScore += 1
        }
        else if (state == aiXO) {
          aiScore += 1
        }
      }
    }
    (playerScore, aiScore)
  }

  def checkGameOver(copiedBoard: ListBuffer[ListBuffer[String]]): String = {

  }

  private def minimax(copiedBoard: ListBuffer[ListBuffer[String]], depth: Int, isMaximizer: Boolean, aiXO: String, playerXO: String): Int = {
    // check the state of each copied board simulated
    val stateValue = evaluateStateScore(copiedBoard, playerXO, aiXO)
    if (depth == 4) {
      return stateValue._1 - stateValue._2
    }
    if (stateValue == )

      if (isMaximizer) {
        var bestScore = Integer.MIN_VALUE
        for (y <- 0 to 7) {
          for (x <- 0 to 7) {
            if (copiedBoard(y)(x) == " ") {
              copiedBoard(y)(x) == aiXO
              val score = minimax(copiedBoard.clone(), depth + 1, false, aiXO, playerXO)
              bestScore = bestScore.max(score)
            }
          }
        }
        bestScore
      }
      else {
        var bestScore = Integer.MAX_VALUE
        for (y <- 0 to 7) {
          for (x <- 0 to 7) {
            if (copiedBoard(y)(x) == " ") {
              copiedBoard(y)(x) == playerXO
              val score = minimax(copiedBoard, depth + 1, true, aiXO, playerXO)
              bestScore = bestScore.min(score)
            }
          }
        }
        bestScore
      }
  }

  def playGame(): Unit = {
    var playing = true

    print("Do u want to be 'X' or 'O' -> ")
    val xOro = scala.io.StdIn.readLine()
    var parsedInput = parseInput(xOro)
    var playerXO = ""
    breakable {
      while (true) {
        if (parsedInput.length == 1) {
          val xo = parsedInput.head.toLowerCase()
          if (xo == "x") {
            println("You are 'X'")
            playerXO = "X"
            break
          }
          else if (xo == "o") {
            println("You are 'O'")
            playerXO = "O"
            break
          }
        }
        println("invalid selection. please pick 'X' or 'O' again.")
        val xOro = scala.io.StdIn.readLine()
        parsedInput = parseInput(xOro)
      }
    }
    val aiXO = if playerXO == "X" then "O" else "X"

    var isYourTurn = math.random() < 1.0 // 1 means your turn 100%
    if (!isYourTurn) {
      println("AI is '" + aiXO + "'. AI goes first. Good luck! ;)\n")
    }
    else {
      println("You are '" + playerXO + "'. Your move first. Good luck! ;)\n")
    }

    while (playing) {
      currentBoard()

      if (!isYourTurn) {
        val aiMove: (Int, Int) = calculateMoveMinimax(aiXO, playerXO) // find best move alpha-beta
        makeMove(aiMove, aiXO)
        println("AI made move '" + aiMove + "'.")
        isYourTurn = true
      }
      else {
        print("Your move -> ")

        val userCommand = scala.io.StdIn.readLine()
        val parsedCommand = parseInput(userCommand)

        if (isValidCommand(parsedCommand)) {
          val command = parsedCommand
          if (command.head.toLowerCase() == "help") {
            helpCommand()
          }
          else if (command.head.toLowerCase() == "quit") {
            playing = false
          }
          else {
            val move = (command(0).toInt, command(1).toInt)
            if (isValidMove(move, playerXO)) {
              makeMove(move, playerXO)
              isYourTurn = false
            }
            else {
              println("already occupied or invalid move. please select valid coordinate.")
            }
          }
        }
        else {
          println("invalid command from player.")
          println("type 'help' for additional info.\n")
        }
      }

    }
    println("thank u for playing.")
  }

  playGame()
}