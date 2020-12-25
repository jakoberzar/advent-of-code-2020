// Learn more about F# at http://fsharp.org
namespace AdventOfCode

module Day25 =
    let inputPath = "./../inputs/day-25.txt"

    let parseInput (path: string) =
        let keys = inputPath.Trim() |> System.IO.File.ReadAllLines |> Seq.map uint64 |> Seq.toList
        match keys with
        | [first; second] -> (first, second)
        | _ -> failwith "Invalid input"

    let transformKey value subjectNumber = value * subjectNumber % 20201227UL

    let findAmountOfLoops key =
        let rec findLoops value iterations =
            if key = value then iterations
            else findLoops (transformKey value 7UL) (iterations + 1)
        findLoops 1UL 0

    let calculateEncryptionKey publicKey iterations =
        seq { 1 .. iterations } |> Seq.fold (fun acc _ -> transformKey acc publicKey) 1UL

    [<EntryPoint>]
    let main argv =
        let (cardKey, doorKey) = parseInput inputPath
        let loopsCard = findAmountOfLoops cardKey
        let encryptionKey = calculateEncryptionKey doorKey loopsCard
        printfn "The encryption key is %i" encryptionKey
        0 // return an integer exit code
