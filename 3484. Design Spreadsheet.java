class Spreadsheet {
    public Spreadsheet(int rows) {
    }
    Map<String, Integer> map = new HashMap<>(); 
    public void setCell(String cell, int value) {
        map.put(cell, value);
    }
    public void resetCell(String cell) {
        map.remove(cell);
    }
    public int getValue(String formula) {
        int io = formula.indexOf('+');
        String cell1 = formula.substring(1, io);     
        String cell2 = formula.substring(io + 1); 
        
        int val1;
        if (cell1.charAt(0) > '9') {
            val1 = map.getOrDefault(cell1, 0);
        } else {
            val1 = Integer.parseInt(cell1);
        }
        int val2;
        if (cell2.charAt(0) > '9') {
            val2 = map.getOrDefault(cell2, 0);
        } else {
            val2 = Integer.parseInt(cell2);
        }
        return val1 + val2;
    }
}
