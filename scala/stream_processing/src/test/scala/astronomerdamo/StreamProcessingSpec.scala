package astronomerdamo

import org.scalatest._
import astronomerdamo.StreamProcessing._

class StreamProcessingSpec extends FlatSpec with Matchers {

  "The process object" should "count groups correctly" in {
    StreamState.processStream("").groupCount shouldEqual 0
    StreamState.processStream("{}").groupCount shouldEqual 1
    StreamState.processStream("{{{}}}").groupCount shouldEqual 6
    StreamState.processStream("{{},{}}").groupCount shouldEqual 5
    StreamState.processStream("{{{},{},{{}}}}").groupCount shouldEqual 16
  }

  "The process object" should "count groups correctly and ignore garbage" in {
    StreamState.processStream("{<a>,<a>,<a>,<a>}").groupCount shouldEqual 1
    StreamState.processStream("{{<ab>},{<ab>},{<ab>},{<ab>}}").groupCount shouldEqual 9
  }

  "The process object" should "count groups correctly and ignore garbage and cancel bangs" in {
    StreamState.processStream("{{<!!>},{<!!>},{<!!>},{<!!>}}").groupCount shouldEqual 9
    StreamState.processStream("{{<a!>},{<a!>},{<a!>},{<ab>}}").groupCount shouldEqual 3
  }

  "The process object" should "count garbage correctly" in {
    StreamState.processStream("<>").garbageCount shouldEqual 0
    StreamState.processStream("<random characters>").garbageCount shouldEqual 17
    StreamState.processStream("<<<<>").garbageCount shouldEqual 3
  }

  "The process object" should "count garbage correctly while ignoring cancel bangs" in {
    StreamState.processStream("<{!>}>").garbageCount shouldEqual 2
    StreamState.processStream("<!!>").garbageCount shouldEqual 0
    StreamState.processStream("<!!!>>").garbageCount shouldEqual 0
    StreamState.processStream("<{o'i!a,<{i<a>").garbageCount shouldEqual 10
  }

}
