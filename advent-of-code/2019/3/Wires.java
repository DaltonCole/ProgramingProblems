import java.util.Scanner;
import java.util.List;
import java.util.Arrays;
import java.util.ArrayList;
import java.util.*;


public class Wires {
    Integer x;
    Integer y;

    public static void parse(String wire[], List<Character> dir, List<Integer> mag) {
        for(int i = 0; i < wire.length; i++) {
            dir.add(wire[i].charAt(0));
            mag.add(Integer.parseInt(wire[i].substring(1, wire[i].length())));
        }
        return;
    }

    public Wires(Integer x, Integer y) {
        this.x = x;
        this.y = y;
    }

    @Override
    public boolean equals(Object other) {
        if(this == other) {
            return true;
        }
        if(other == null) {
            return false;
        }
        if(getClass() != other.getClass()) {
            return false;
        }
        Wires wires = (Wires) other;
        return Objects.equals(x, wires.x) && Objects.equals(y, wires.y);
    }

    @Override
    public int hashCode() {
        String tmp = String.valueOf(this.x) + " " + String.valueOf(this.y);
        return tmp.hashCode();
    }

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        String wire1[] = scanner.nextLine().split(",");
        String wire2[] = scanner.nextLine().split(",");

        List<Character> dir1 = new ArrayList<Character>();
        List<Character> dir2 = new ArrayList<Character>();
        List<Integer> mag1 = new ArrayList<Integer>();
        List<Integer> mag2 = new ArrayList<Integer>();

        parse(wire1, dir1, mag1);
        parse(wire2, dir2, mag2);

        System.out.println(dir1);
        System.out.println(mag1);

        HashMap<Wires, Integer> pair = new HashMap<>();

        int x = 0;
        int y = 0;
        int dist = 0;
        for(int i = 0; i < dir1.size(); i++) {
            if(dir1.get(i) == 'U') {
                for(int j = 0; j < mag1.get(i); j++) {
                    y++;
                    dist++;
                    pair.put(new Wires(x, y), dist);
                } 
            } else if(dir1.get(i) == 'D') {
                for(int j = 0; j < mag1.get(i); j++) {
                    y--;
                    dist++;
                    pair.put(new Wires(x, y), dist);
                }
            } else if(dir1.get(i) == 'R') {
                for(int j = 0; j < mag1.get(i); j++) {
                    x++;
                    dist++;
                    pair.put(new Wires(x, y), dist);
                }
            } else {
                for(int j = 0; j < mag1.get(i); j++) {
                    x--;
                    dist++;
                    pair.put(new Wires(x, y), dist);
                }
            }
        }

        int min_distance = 9999999;
        Wires tmp = new Wires(0,0);
        Integer old_dist;
        dist = 0;

        for(int i = 0; i < dir2.size(); i++) {
            if(dir2.get(i) == 'U') {
                for(int j = 0; j < mag2.get(i); j++) {
                    tmp.y++;
                    dist++;
                    old_dist = pair.get(tmp);
                    if(old_dist != null) {
                        min_distance = Math.min(min_distance, dist + old_dist);
                    }
                } 
            } else if(dir2.get(i) == 'D') {
                for(int j = 0; j < mag2.get(i); j++) {
                    tmp.y--;
                    dist++;
                    old_dist = pair.get(tmp);
                    if(old_dist != null) {
                        min_distance = Math.min(min_distance, dist + old_dist);
                    }
                }
            } else if(dir2.get(i) == 'R') {
                for(int j = 0; j < mag2.get(i); j++) {
                    tmp.x++;
                    dist++;
                    old_dist = pair.get(tmp);
                    if(old_dist != null) {
                        min_distance = Math.min(min_distance, dist + old_dist);
                    }
                }
            } else {
                for(int j = 0; j < mag2.get(i); j++) {
                    tmp.x--;
                    dist++;
                    old_dist = pair.get(tmp);
                    if(old_dist != null) {
                        min_distance = Math.min(min_distance, dist + old_dist);
                    }
                }
            }
        }

        System.out.println(min_distance);
    }
}
