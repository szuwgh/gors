package main

// export LD_LIBRARY_PATH=/opt/rsproject/gors/rust/target/debug

/*
#cgo CFLAGS: -I.
#cgo LDFLAGS: -L../rust/target/debug -lrust
#include "ffi_demo.h"
*/
import "C"
import "fmt"

func main() {
	arg1 := 1
	arg2 := 2
	arg3 := 3

	cArg1 := C.uint8_t(arg1)
	cArg2 := C.uint16_t(arg2)
	cArg3 := C.uint32_t(arg3)
	ret := C.simple_rust_func_called_from_go(cArg1, cArg2, cArg3)
	if int(ret) != arg1+arg2+arg3 {
		panic("SimpleRustFuncCalledFromGo Error")
	}
	fmt.Println(ret)
}