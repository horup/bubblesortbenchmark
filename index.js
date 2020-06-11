let num = 50*1000;
let array = [];
for (var i = 0; i < num; i++)
    array[i] = array.length - i;

let rerun = 10;
let tick = new Date().getTime();

for (let r = 0; r < rerun; r++)
{
    for (let i = 0; i < array.length; i++)
        for (let j = 0; j < array.length; j++)
        {
            if (array[i] < array[j])
            {
                let org = array[i];
                array[i] = array[j];
                array[j] = org;
            }
        }
}

let took = new Date().getTime() - tick;

console.log(`${took}ms to sort ${array.length} elements ${rerun} times`);