package astronomerdamo

case class StreamState (
  groupCount: Int,
  depthCount: Int,
  garbageCount: Int,
  isCancel: Boolean,
  isGarbage: Boolean,
) {
  def incrGarbage(state: StreamState): StreamState = state.copy(garbageCount = garbageCount + 1)

  def groupState(state: StreamState, incr: Boolean): StreamState = {
    if (state.isGarbage) {
      state.incrGarbage(state)
    } else if (incr) {
      state.copy(depthCount = depthCount + 1)
    } else {
      state.copy(groupCount = groupCount + depthCount, depthCount = depthCount - 1)
    }
  }

  def garbageState(state: StreamState, incr: Boolean): StreamState = {
    if (state.isGarbage && incr) {
      state.incrGarbage(state)
    } else if (!state.isGarbage && incr) {
      state.copy(isGarbage = true)
    } else {
      state.copy(isGarbage = false)
    }
  }

  def machine(state: StreamState, c: Char): StreamState = {
    c match {
      case '!' => state.copy(isCancel = true)
      case '{' => state.groupState(state, true)
      case '}' => state.groupState(state, false)
      case '<' => state.garbageState(state, true)
      case '>' => state.garbageState(state, false) 
      case _   => if (state.isGarbage) state.incrGarbage(state) else state
    }
  }
}

case object StreamState {
  val empty = StreamState(0, 0, 0, false, false)
  def processStream(text: String, debug: Boolean = false): StreamState = text.foldLeft(StreamState.empty)((state, c) => {
    val newState = if (state.isCancel) {
      state.copy(isCancel = false)
    } else {
      state.machine(state, c)
    }
    if (debug) println("%s, %s".format(c, newState))
    newState
  })
}

object StreamProcessing extends App {
  val streamState = StreamState.processStream(args.head.mkString)
  println("Group Count: %d, Garbage Count %d".format(streamState.groupCount, streamState.garbageCount))
}
