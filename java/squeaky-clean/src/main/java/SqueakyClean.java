class SqueakyClean {
    static String clean(String identifier) {
        StringBuilder result = new StringBuilder();
        boolean nextUpper = false;

        for (int i = 0; i < identifier.length(); i++) {
            char ch = identifier.charAt(i);

            if (ch == ' ') {
                result.append('_');
            } else if (ch == '-') {
                nextUpper = true;
            } else if (nextUpper) {
                result.append(Character.toUpperCase(ch));
                nextUpper = false;
            } else {
                switch (ch) {
                case '4': result.append('a'); break;
                case '3': result.append('e'); break;
                case '0': result.append('o'); break;
                case '1': result.append('l'); break;
                case '7': result.append('t'); break;
                default:
                    if (Character.isLetter(ch)) result.append(ch);
                }
            }
        }
        
        return result.toString();
    }
}
