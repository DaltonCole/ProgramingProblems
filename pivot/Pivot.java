import java.util.Scanner;

public class Pivot {
	public static void main(String[] args) {
		Scanner s = new Scanner(System.in);

		int number = s.nextInt();

		int[] smaller = new int [number];
		int[] larger = new int [number];
		int[] container = new int [number];

		int temp;

		for(int i = 0; i < number; i++) {
			container[i] = s.nextInt();
		}

		temp = 0;
		for(int i = 0; i < number; i++) {
			if(temp < container[i]) {
				temp = container[i];
			}
			smaller[i] = temp;
		}

		temp = 999999;
		for(int i = number - 1; i >= 0; i--) {
			if(temp > container[i]) {
				temp = container[i];
			}
			larger[i] = temp;
		}

		temp = 0;
		for(int i = 0; i < number; i++) {
			if(smaller[i] <= container[i] && larger[i] >= container[i]) {
				temp++;
			}
		}

		System.out.println(temp);
	}
}