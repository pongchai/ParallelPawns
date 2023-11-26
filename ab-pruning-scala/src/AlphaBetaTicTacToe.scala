package working

import working.AlphaBetaTicTacToe.board

import scala.annotation.tailrec
import scala.collection.mutable
import scala.concurrent.ExecutionContext.Implicits.global
import scala.concurrent.duration.Duration
import scala.concurrent.{Await, Future}
import scala.util.control.Breaks.{break, breakable}

object AlphaBetaTicTacToe extends App {
  private val board = mutable.HashMap(
    1 -> " ", 2 -> " ", 3 -> " ",
    4 -> " ", 5 -> " ", 6 -> " ",
    7 -> " ", 8 -> " ", 9 -> " ",
  )

  var simulations = 0

  def calculateMoveMini(aiXO: String, playerXO: String): Int = {
    // calculating best possible move
    var bestScore = Integer.MIN_VALUE
    var bestMove = -1
    val alpha = Integer.MIN_VALUE
    val beta = Integer.MAX_VALUE
    for (coord <- board.keys) {
      if (board(coord) == " ") {
        board.update(coord, aiXO)
        val score = minimax(0, false, aiXO, playerXO)
        board.update(coord, " ")
        if (score > bestScore) {
          bestScore = score
          bestMove = coord
        }
      }
    }
    println("end game simulations went thru: " + simulations)
    simulations = 0
    bestMove
  }

  def calculateMoveAlpha(aiXO: String, playerXO: String): Int = {
    var bestScore = Integer.MIN_VALUE
    var bestMove = -1
    for (coord <- board.keys) {
      if (board(coord) == " ") {
        board.update(coord, aiXO)
        val score = alphaBetaPruning(aiXO, playerXO)
        board.update(coord, " ")
        if (score > bestScore) {
          bestScore = score
          bestMove = coord
        }
      }
    }
    println("end game simulations went thru: " + simulations)
    simulations = 0
    bestMove
  }


  def alphaBetaPruning(aiXO: String, playerXO: String): Int = {
    def maxValue(alpha: Int, beta: Int): Int = {
      val checkedWin = checkWin().getOrElse("None")
      if (checkedWin == aiXO) {
        simulations += 1
        return 1
      }
      else if (checkedWin == playerXO) {
        simulations += 1
        return -1
      }
      else if (checkDraw()) {
        simulations += 1
        return 0
      }
      var bestScore = Integer.MIN_VALUE
      var a = alpha
      breakable {
        for (coord <- board.keys) {
          if (board(coord) == " ") {
            board.update(coord, aiXO)
            bestScore = bestScore.max(minValue(a, beta))
            board.update(coord, " ")
            a = a.max(bestScore)
            if (bestScore >= beta) {
              break
            }
          }
        }
      }
      bestScore
    }

    def minValue(alpha: Int, beta: Int): Int = {
      val checkedWin = checkWin().getOrElse("None")
      if (checkedWin == aiXO) {
        simulations += 1
        return 1
      }
      else if (checkedWin == playerXO) {
        simulations += 1
        return -1
      }
      else if (checkDraw()) {
        simulations += 1
        return 0
      }
      var bestScore = Integer.MAX_VALUE
      var b = beta
      breakable {
        for (coord <- board.keys) {
          if (board(coord) == " ") {
            board.update(coord, playerXO)
            bestScore = bestScore.min(maxValue(alpha, b))
            board.update(coord, " ")
            b = b.min(bestScore)
            if (bestScore <= alpha) {
              break
            }
          }
        }
      }
      bestScore
    }
    minValue(Integer.MIN_VALUE, Integer.MAX_VALUE)
  }

  def minimax(depth: Int, isMaximizer: Boolean, aiXO: String, playerXO: String): Int = {
    val checkedWin = checkWin().getOrElse("None")
    if (checkedWin == aiXO) { // if AI xo won
      simulations += 1
      return 1
    }
    else if (checkedWin == playerXO) { // if player xo won
      simulations += 1
      return -1
    }
    else if (checkDraw()) { // if a draw
      simulations += 1
      return 0
    }
    // continue to below if not yet any of the 3 states above
    if (isMaximizer) {
      var bestScore = Integer.MIN_VALUE
      for (coord <- board.keys) {
        if (board(coord) == " ") {
          board.update(coord, aiXO)
          val score = minimax(depth + 1, false, aiXO, playerXO)
          board.update(coord, " ")
          bestScore = bestScore.max(score)
        }
      }
      bestScore
    }
    else {
      var bestScore = Integer.MAX_VALUE
      for (coord <- board.keys) {
        if (board(coord) == " ") {
          board.update(coord, playerXO)
          val score = minimax(depth + 1, true, aiXO, playerXO)
          board.update(coord, " ")
          bestScore = bestScore.min(score)
        }
      }
      bestScore
    }
  }

  def checkWin(): Option[String] = { // check if someone won -> who won
    if (board(1) == board(2) && board(2) == board(3)) {
      Some(board(3))
    }
    else if (board(4) == board(5) && board(5) == board(6)) {
      Some(board(6))
    }
    else if (board(7) == board(8) && board(8) == board(9)) {
      Some(board(9))
    }
    else if (board(1) == board(4) && board(4) == board(7)) {
      Some(board(7))
    }
    else if (board(2) == board(5) && board(5) == board(8)) {
      Some(board(8))
    }
    else if (board(3) == board(6) && board(6) == board(9)) {
      Some(board(9))
    }
    else if (board(1) == board(5) && board(5) == board(9)) {
      Some(board(9))
    }
    else if (board(3) == board(5) && board(5) == board(7)) {
      Some(board(7))
    }
    else None
  }

  def checkDraw(): Boolean = {
    var count = 9
    for (coord <- board.keys) {
      if count <= 2 then return true
      if (board(coord) == " ") {
        return false
      }
      count -= 1
    }
    true
  }

  def makeMove(move: Int, isPlayer: Boolean, xo: String): Unit = {
    if (isPlayer) { // player's move
      board.update(move, xo)
    }
    else { // ai's move
      board.update(move, xo)
    }
  }

  def isValidMove(move: Int): Boolean = {
    // simply a free space on board
    if board(move) == " " then true else false
  }

  def currentBoard(): Unit = {
    println("\n" + board(1) + " | " + board(2) + " | " + board(3))
    println("--+---+--")
    println(board(4) + " | " + board(5) + " | " + board(6))
    println("--+---+--")
    println(board(7) + " | " + board(8) + " | " + board(9) + "\n")
  }

  def coordinationsBoard(): Unit = {
    println("\n1 | 2 | 3")
    println("--+---+--")
    println("4 | 5 | 6")
    println("--+---+--")
    println("7 | 8 | 9\n")
  }

  def parseInput(input: String): Array[String] = {
    input.trim.split(" ")
  }

  def isValidCommand(commands: Array[String]): Boolean = {
    if (commands.length == 1) {
      val command = commands.head.toLowerCase()
      if (command == "help" || command == "quit" || isCoordMove(command)) {
        return true
      }
    }
    false
  }

  def helpCommand(): Unit = {
    println("\nAVAILABLE COMMANDS:")
    println("- 'help'")
    println("- 'quit'")
    println("- select between 1-9 corresponding to board spots below")
    coordinationsBoard()
    println()
  }

  def isCoordMove(move: String): Boolean = { // check coordinate moves
    val moves = List("1","2","3","4","5","6","7","8","9")
    moves.contains(move)
  }

  def playGame(): Unit = {
    var playing = true

    println("select an algorithm:")
    println("1. minimax")
    println("2. alpha-beta pruning")
    val inp = scala.io.StdIn.readLine()
    var parsedInput = parseInput(inp)
    var selectedAlgo = ""
    breakable {
      while (true) {
        if (parsedInput.length == 1) {
          selectedAlgo = parsedInput.head.toLowerCase()
          if (selectedAlgo == "1" || selectedAlgo == "2") {
            break
          }
        }
        println("invalid selection. select 1 or 2")
        val inp = scala.io.StdIn.readLine()
        parsedInput = parseInput(inp)
      }
    }

    print("Do u want to be 'X' or 'O' -> ")
    val xOro = scala.io.StdIn.readLine()
    parsedInput = parseInput(xOro)
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

    var isYourTurn = math.random() < 0.0 // 50% chance for who's turn
    if (!isYourTurn) {
      println("AI is '" + aiXO + "'. AI goes first. Good luck! ;)\n")
    }
    else {
      println("You are '" + playerXO + "'. Your move first. Good luck! ;)\n")
    }
    coordinationsBoard()
    while (playing) {
      currentBoard()

      if (!isYourTurn) { // computer's turn
        if (selectedAlgo == "1") {
          val aiMove = calculateMoveMini(aiXO, playerXO) // using minimax
          makeMove(aiMove, !isYourTurn, aiXO)
          println("AI made move '" + aiMove + "'.")
          isYourTurn = true
        }
        else if (selectedAlgo == "2") {
          val aiMove = calculateMoveAlpha(aiXO, playerXO) // using alpha-beta
          makeMove(aiMove, !isYourTurn, aiXO)
          println("AI made move '" + aiMove + "'.")
          isYourTurn = true
        }
      }
      else { // your turn
        print("Your move -> ")
        val userCommand = scala.io.StdIn.readLine()
        val parsedCommand = parseInput(userCommand)

        if (isValidCommand(parsedCommand)) {
          val command = parsedCommand.head.toLowerCase()
          if (command == "help") {
            helpCommand()
          }
          else if (command == "quit") {
            playing = false
          }
          else {
            val move = command.toInt // has a num between 1-9
            if (isValidMove(move)) {
              makeMove(move, isYourTurn, playerXO)
              isYourTurn = false
            }
            else {
              println("already occupied. please select valid coordinate between 1-9.\n")
            }
          }
        }
        else {
          println("invalid command from player.")
          println("type 'help' for additional info.\n")
        }
      }
      val checkedWin = checkWin().getOrElse("None")
      if (checkedWin == aiXO) {
        println("AI won. -1 for humanity.")
        currentBoard()
        playing = false
      }
      else if (checkedWin == playerXO) {
        println("You won! +1 for humanity.")
        currentBoard()
        playing = false
      }
      else if (checkDraw()) {
        println("a Draw.")
        currentBoard()
        playing = false
      }
    }
    println("thank u for playing.")
  }

  playGame()
}
