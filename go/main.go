package main

import (
	"fmt"
	"reflect"
	"unsafe"
)

// export LD_LIBRARY_PATH=/opt/rsproject/gors/rust/target/debug

/*
#cgo CFLAGS: -I.
#cgo LDFLAGS: -L../rust/target/debug -lrust
#include "ffi.h"
*/
import "C"

func main() {
	var t1 = []byte{'a', 'b', 'c'}
	var t2 = []byte{'1', '2', '3'}
	b := C.new_builder()
	C.add_key(b, (*C.uchar)(unsafe.Pointer(&t1[0])), C.uint(len(t1)))
	C.add_key(b, (*C.uchar)(unsafe.Pointer(&t2[0])), C.uint(len(t2)))

	char := C.get(b)
	length := C.len(b)

	var data []byte
	h := (*reflect.SliceHeader)((unsafe.Pointer(&data)))
	h.Data = uintptr(unsafe.Pointer(char))
	h.Len = int(length)
	h.Cap = int(length)
	fmt.Println("data", data)

}
