fn main() {

    const SEGMENT_SIZE: usize = 65536; //size of the segment

    let mut primes: Vec<u64> = Vec::new(); // the vector which stores primes of previous sectors
    let mut offset:     u64  = 0;          // the offset of the current sector


    {   //first run to establish the first segment of primes
        //creates segment
        let mut segment: [bool; SEGMENT_SIZE] = [true; SEGMENT_SIZE];

        segment[0]=false;
        segment[1]=false;

        //marks all numbers in the sector that are a multiple of a prime
        for i in 2..(SEGMENT_SIZE as f64).sqrt() as usize {

            if segment[i]==false { continue; } //skips numbers that are not a prime

            for j in i..SEGMENT_SIZE/i+1 {

                if j*i >= SEGMENT_SIZE { continue; } // skips if number is not in the segment

                segment[i*j]=false;

            }

        }
        //read primes from segment and push into
        for i in 1..SEGMENT_SIZE {
            if segment[i] {
                primes.push(i as u64);
                println!("{}",i);
            }
        }

        offset = offset + SEGMENT_SIZE as u64;
    }


    //loop 2...infinity
    loop {
        println!("------------------- {} - {}",offset, offset+SEGMENT_SIZE as u64);
        let mut segment: [bool; SEGMENT_SIZE] = [true; SEGMENT_SIZE];

        // uses all known primes from previous runs to calculate non-primes
        for i in &primes {
            for j in ( offset/ i)..( SEGMENT_SIZE as u64 + offset ) / i +1 {

                if i*j<offset || i*j >= offset+SEGMENT_SIZE as u64 { continue } //skip if value isn't in the segment

                segment[(i*j-offset) as usize]=false;

            }

        }

        //reads primes from segment, pushes them into the storage
        for i in 1..SEGMENT_SIZE {
            if segment[i] {
                primes.push(i as u64+offset );
                println!("{}",i as u64+offset );
            }
        }


        offset = offset + SEGMENT_SIZE as u64;

        /*
        if offset>=10000000.0 {
            break;
        }
        */
    }

}
