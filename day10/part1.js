let inputA=
`noop
addx 3
addx -5`;


let inputB=
`addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop`;

let input1 = 
`addx 2
addx 3
noop
addx 1
addx 27
addx -23
addx 5
noop
addx 1
noop
addx 4
addx 1
noop
addx 4
addx 5
noop
noop
noop
addx 5
addx -4
addx 4
noop
addx 1
addx -38
noop
noop
addx 7
addx 8
addx -3
noop
addx 3
noop
addx 5
noop
noop
addx -2
addx 2
addx 9
addx -2
addx 6
addx 1
addx -4
addx 5
addx 2
addx -14
addx -6
addx -16
addx 1
addx 5
addx 1
addx 4
addx -2
noop
addx -7
addx -3
addx 17
addx 5
noop
noop
addx 19
addx -16
noop
addx 14
addx -8
addx 2
noop
addx 4
noop
addx -35
addx -2
noop
noop
addx 7
addx 19
addx -26
addx 10
addx 29
addx -21
noop
addx 4
noop
noop
addx -9
addx 4
addx 8
addx 7
noop
addx -2
addx 5
addx 2
addx -19
addx -18
noop
noop
noop
noop
addx 7
addx -7
addx 37
addx -27
addx 5
addx 2
addx -12
addx 4
addx 11
noop
noop
noop
addx 5
addx -14
addx 21
addx -4
addx 5
addx 2
noop
addx -35
noop
noop
noop
noop
addx 7
addx 1
noop
noop
addx 5
addx -1
addx 5
addx 1
noop
addx 4
addx 1
noop
noop
addx 4
noop
addx 1
addx 2
addx 5
addx 2
addx 1
noop
noop
noop
noop`;

let C = input1.split("\n").map(r=>r.split(' ')[1] || 0)
.reduce((a,n)=>{
    if(n != 0){
        a.push(0)
    }
    a.push(n)
    return a
},[])

let D = C.reduce((agg,n,i)=>{
    agg.push(parseInt(n)+parseInt(agg[i]))
    // console.log(n, i, agg[i])
    return agg
},[1])
console.log('state', D)
let N = C.map((e,i)=>i+1);
let SS = D.map((e,i)=>e*(i+1))

console.log('cycle', N)
console.log('sign ',SS)
//console.log(SS[19])
let sum = N.reduce((a,n,i)=>{
    if((n - 19) % 40 == 0){
        a += SS[i+1]
    }
    return a
},0)
console.log(sum)




// PART 2
let crt = D.reduce((a,n,i)=>{
 let row = Math.floor((i) / 40)
 let col = (i) % 40;
 let x = ''
 if(a[row]){
    if(Math.abs(D[i]-col)<=1){
        x = '#';
    }else{
        x = '.';
    } 
    console.log('drawing: ', col, 'regsiter: ',D[i], 'result: ', x)

    a[row].push(x)
 }


 return a
},[[],[],[],[],[],[]]).map(row=>row.join(''))

console.log(crt)