import java.util.Scanner;

public class Railroad {
	public static void main(String[] args) {
		Scanner s = new Scanner(System.in);

		int x = s.nextInt();
		int y = s.nextInt();

		int total;

		total = x * 4;
		total += (y * 3);

		if(total % 2 == 0) {
			System.out.println("possible");
		} else {
			System.out.println("impossible");
		}

		return;
	}
}