import java.util.Scanner;

public class Bishop {
	public static void main(String[] args) {
		Scanner s = new Scanner(System.in);

		int n;

		while(s.hasNextInt()) {
			n = s.nextInt();

			if(n != 1) {
				System.out.println((2 * n) - 2);
			} else {
				System.out.println(1);
			}
		}

		return;
	}
}