import java.util.HashMap;
import java.util.Arrays;
import java.util.Scanner;

public class OpCode {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        String data[] = scanner.nextLine().split(",");

        long[] numbers = new long[data.length];
        for(int i = 0; i < data.length; i++) {
            numbers[i] = Integer.parseInt(data[i]);
        }

        scanner.close();
        
        long[] numbers2 = numbers.clone();

        for(int x = 0; x < 100; x++) {
            for(int y = 0; y < 100; y++) {
                numbers = numbers2.clone();
                numbers[1] = x;
                numbers[2] = y;
                for(int i = 0; i < numbers.length; i += 4) {
                    if(numbers[i] == 1) {
                        numbers[(int)numbers[i+3]] = numbers[(int)numbers[i+1]] + numbers[(int)numbers[i+2]];
                    } else if(numbers[i] == 2) {
                        numbers[(int)numbers[i+3]] = numbers[(int)numbers[i+1]] * numbers[(int)numbers[i+2]];
                    } else {
                        break;
                    }
                }
                if(numbers[0] == 19690720) {
                    System.out.println(numbers[1]);
                    System.out.println(numbers[2]);
                    System.out.println((100 * numbers[1]) + numbers[2]);
                    return;
                }
            }
        }
    }
}
