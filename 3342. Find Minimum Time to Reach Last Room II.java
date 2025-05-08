class Solution {
	private static class Room implements Comparable<Room> {
		final int openTime;
		final boolean longStay;
		Room[] adjacent;
		Room next;
		Room() {
			openTime = Integer.MAX_VALUE;
			longStay = true;
		}
		Room(int openTime, boolean longStay) {
			this.openTime = openTime;
			this.longStay = longStay;
			next = this;
		}
		@Override
		public int compareTo(Room other) {
			return openTime - other.openTime;
		}
	}
	private static final Room DUMMY_ROOM = new Room();
	private static Room initRooms(int[][] moveTime) {
		int n = moveTime.length;
		int m = moveTime[0].length;
		Room[][] rooms = new Room[n][m];
		for (int i = 0; i < n; i++) {
			int[] mtRow = moveTime[i];
			Room[] rRow = rooms[i];
			for (int j = 0; j < m; j++)
				rRow[j] = new Room(mtRow[j], ((i + j) & 1) == 0);
		}
		Room[] dummyRow = new Room[m];
		Arrays.fill(dummyRow, DUMMY_ROOM);
		Room[] prevRow = dummyRow;
		Room[] curRow = rooms[0];
		n--;
		m--;
		for (int i = 0; i <= n; i++) {
			Room[] nextRow = i < n ? rooms[i + 1] : dummyRow;
			Room prev = DUMMY_ROOM;
			Room cur = curRow[0];
			for (int j = 0; j <= m; j++) {
				Room next = j < m ? curRow[j + 1] : DUMMY_ROOM;
				cur.adjacent = new Room[] { prev, prevRow[j], next, nextRow[j] };
				prev = cur;
				cur = next;
			}
			prevRow = curRow;
			curRow = nextRow;
		}
		Room start = rooms[0][0];
		start.next = rooms[n][m];
		return start;
	}
	public static int minTimeToReach(int[][] moveTime) {
		Room start = initRooms(moveTime);
		Room finish = start.next;
		Queue<Room> waitingToEnter = new PriorityQueue<>();
		waitingToEnter.add(DUMMY_ROOM);
		start.next = null;
		Room exitingShortHead = start;
		Room exitingLongHead = null;
		int currentTime = 0;
		while (true) {
			Room exitingLongHeadNew = null;
			while (exitingShortHead != null) {
				for (Room adj : exitingShortHead.adjacent)
					if (adj.next == adj) {
						if (adj == finish)
							return Math.max(currentTime, finish.openTime) + (finish.longStay ? 2 : 1);
						if (adj.openTime <= currentTime) {
							if (adj.longStay) {
								adj.next = exitingLongHeadNew;
								exitingLongHeadNew = adj;
							} else {
								adj.next = exitingLongHead;
								exitingLongHead = adj;
							}
						} else {
							adj.next = null;
							waitingToEnter.offer(adj);
						}
					}
				exitingShortHead = exitingShortHead.next;
			}
			exitingShortHead = exitingLongHead;
			exitingLongHead = exitingLongHeadNew;
			int queueTime;
			while ((queueTime = waitingToEnter.peek().openTime) <= currentTime) {
				Room entering = waitingToEnter.poll();
				if (entering.longStay) {
					entering.next = exitingLongHead;
					exitingLongHead = entering;
				} else {
					entering.next = exitingShortHead;
					exitingShortHead = entering;
				}
			}
			if (++currentTime < queueTime && exitingShortHead == null && exitingLongHead == null)
				currentTime = queueTime;
		}
	}
}
