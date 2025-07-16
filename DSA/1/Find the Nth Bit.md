# Find the Nth bit

- This is used to find the ith bit in the n

```java
import java.io.*;

import java.util.*;

  

public class Main {

  

public static void main(String[] args) {

Scanner sc=new Scanner(System.in);

int N=sc.nextInt();

int i=sc.nextInt();



System.out.println( "The ith bit is"+N&(1<<i));



}

```

