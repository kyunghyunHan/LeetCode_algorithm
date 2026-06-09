int numJewelsInStones(char* jewels, char* stones) {
    int count = 0;

    for (int i = 0; stones[i] != '\0'; i++) {
        for (int j = 0; jewels[j] != '\0'; j++) {
            if (stones[i] == jewels[j]) {
                count++;
                break;
            }
        }
    }

    return count;
}