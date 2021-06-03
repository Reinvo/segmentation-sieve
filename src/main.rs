fn main() {
    const SEGMENT_SIZE: f64 = 65536.0; //size of the segment

    let mut primes: Vec<usize> = Vec::new(); // the vector which stores primes of previous sectors
    let mut offset: f64 = 0.0;               // the offset of the current sector


    {   //first run to establish the first segment of primes
        //creates segment
        let mut segment: [bool; SEGMENT_SIZE as usize] = [true; SEGMENT_SIZE as usize];

        segment[0]=false;
        segment[1]=false;

        //marks all numbers in the sector that are a multiple of a prime
        for i in 2..(SEGMENT_SIZE).sqrt() as usize {

            if segment[i]==false { continue; } //skips numbers that are not a prime

            for j in i..(SEGMENT_SIZE) as usize/i+1 {

                if j*i >= SEGMENT_SIZE as usize { continue; } // skips if number is not in the segment

                segment[i*j]=false;

            }

        }
        //read primes from segment and push into
        for i in 1..SEGMENT_SIZE as usize {
            if segment[i] {
                primes.push(i);
                println!("{}",i);
            }
        }

        offset = offset + SEGMENT_SIZE;
    }


    //loop 2...infinity
    loop {
        println!("------------------- {} - {}",offset, offset+SEGMENT_SIZE);
        let mut segment: [bool; SEGMENT_SIZE as usize] = [true; SEGMENT_SIZE as usize];

        // uses all known primes from previous runs to calculate non-primes
        for i in &primes {
            for j in (offset/ *i as f64) as usize..(SEGMENT_SIZE+offset) as usize/i+1 {

                if i*j<offset as usize || i*j>=(offset+SEGMENT_SIZE) as usize { continue } //skip if value isn't in the segment

                segment[i*j-offset as usize]=false;

            }

        }

        //reads primes from segment, pushes them into the storage
        for i in 1..SEGMENT_SIZE as usize {
            if segment[i] {
                primes.push(i+offset as usize);
                println!("{}",i+offset as usize);
            }
        }


        offset = offset + SEGMENT_SIZE;

        /*
        if offset>=10000000.0 {
            break;
        }
        */
    }

}
