package astronomerdamo

case class StreamState (
  groupCount: Int,
  currentDepth: Int,
  currentCancel: Boolean,
  currentGarbage: Boolean,
) {

  def matchOpenGroupChar(c: Char): Int = {
    c match {
      case '{' => 1
      case '}' => -1
      case _   => 0
    }
  }

  def matchCloseGroupChar(c: Char): Boolean = {
    c match {
      case '}' => true
      case _   => false
    }
  }

  def matchCancelChar(c: Char): Boolean = {
    c match {
      case '!' => true
      case _   => false
    }
  }

  def matchGarbageChar(c: Char, state: StreamState): Boolean = {
    c match {
      case '<' => true
      case '>' => false
      case _   => state.currentGarbage
    }
  }

  def updateGroupCount(c: Char, state: StreamState): Int = {
    val newGroupScore = if (matchCloseGroupChar(c)) state.currentDepth else 0
    state.groupCount + newGroupScore
  }

  def updateCurrentDepth(c: Char, state: StreamState): Int = {
    state.currentDepth + state.matchOpenGroupChar(c)
  }
}

case object StreamState {
  val empty = StreamState(0, 0, false, false)
}

object StreamProcessing extends App {
  println(
    args.head.foldLeft(StreamState.empty)((state, c) => {
      val isCancel = state.currentCancel
      val isGarbage = state.currentGarbage
      StreamState(
        if (isCancel || isGarbage) state.groupCount else state.updateGroupCount(c, state),
        if (isCancel || isGarbage) state.currentDepth else state.updateCurrentDepth(c, state),
        if (isCancel) false else state.matchCancelChar(c),
        if (isCancel) state.currentGarbage else state.matchGarbageChar(c, state)
      )
    }).groupCount
  )
}
