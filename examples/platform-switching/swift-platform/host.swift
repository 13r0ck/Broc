import Foundation

@_cdecl("broc_alloc")
func brocAlloc(size: Int, _alignment: UInt) -> UInt  {
    guard let ptr = malloc(size) else {
        return 0
    }
    return UInt(bitPattern: ptr)
}

@_cdecl("broc_dealloc")
func brocDealloc(ptr: UInt, _alignment: UInt)  {
    free(UnsafeMutableRawPointer(bitPattern: ptr))
}

@_cdecl("broc_realloc")
func brocRealloc(ptr: UInt, _oldSize: Int, newSize: Int, _alignment: UInt) -> UInt {
    guard let ptr = realloc(UnsafeMutableRawPointer(bitPattern: ptr), newSize) else {
        return 0
    }
    return UInt(bitPattern: ptr)
}

func isSmallString(brocStr: BrocStr) -> Bool {
    return brocStr.capacity < 0
}

func getStrLen(brocStr: BrocStr) -> Int {
    if isSmallString(brocStr: brocStr) {
        // Small String length is last in the byte of capacity.
        var cap = brocStr.capacity
        let count = MemoryLayout.size(ofValue: cap)
        let bytes = Data(bytes: &cap, count: count)
        let lastByte = bytes[count - 1]
        return Int(lastByte ^ 0b1000_0000)
    } else {
        return brocStr.len
    }
}

func getSwiftString(brocStr: BrocStr) -> String {
    let length = getStrLen(brocStr: brocStr)
    
    if isSmallString(brocStr: brocStr) {
        let data: Data = withUnsafePointer(to: brocStr) { ptr in
            Data(bytes: ptr, count: length)
        }
        return String(data: data, encoding: .utf8)!
    } else {
        let data = Data(bytes: brocStr.bytes, count: length)
        return String(data: data, encoding: .utf8)!
    }
}

@_cdecl("main")
func main() -> UInt8 {
    var brocStr = BrocStr()
    broc__mainForHost_1_exposed_generic(&brocStr)
    
    print(getSwiftString(brocStr: brocStr), terminator: "")
    return 0
}
