package goid

import (
	"github.com/go-eden/common/goid/g"
	"runtime"
	"strings"
	"unsafe"
)

var offsetDictMap = map[string]int64{
	"go1.4":     128,
	"go1.4.1":   128,
	"go1.4.2":   128,
	"go1.4.3":   128,
	"go1.5":     184,
	"go1.5.1":   184,
	"go1.5.2":   184,
	"go1.5.3":   184,
	"go1.5.4":   184,
	"go1.6":     192,
	"go1.6.1":   192,
	"go1.6.2":   192,
	"go1.6.3":   192,
	"go1.6.4":   192,
	"go1.7":     192,
	"go1.7.1":   192,
	"go1.7.2":   192,
	"go1.7.3":   192,
	"go1.7.4":   192,
	"go1.7.5":   192,
	"go1.7.6":   192,
	"go1.8":     192,
	"go1.8.1":   192,
	"go1.8.2":   192,
	"go1.8.3":   192,
	"go1.8.4":   192,
	"go1.8.5":   192,
	"go1.9":     152,
	"go1.9.1":   152,
	"go1.9.2":   152,
	"go1.10":    152,
	"go1.10.1":  152,
	"go1.10.2":  152,
	"go1.10.3":  152,
	"go1.10.4":  152,
	"go1.10.5":  152,
	"go1.10.6":  152,
	"go1.10.7":  152,
	"go1.10.8":  152,
	"go1.11":    152,
	"go1.11.1":  152,
	"go1.11.2":  152,
	"go1.11.3":  152,
	"go1.11.4":  152,
	"go1.11.5":  152,
	"go1.11.6":  152,
	"go1.11.7":  152,
	"go1.11.8":  152,
	"go1.11.9":  152,
	"go1.11.10": 152,
	"go1.11.11": 152,
	"go1.12":    152,
	"go1.12.1":  152,
	"go1.12.2":  152,
	"go1.12.3":  152,
	"go1.12.4":  152,
	"go1.12.5":  152,
	"go1.12.6":  152,
	"go1.12.7":  152,
	"go1.13":    152,
	"go1.14":    152,
	"go1.15":    152,
	"go1.16":    152,
}
var offset uintptr

func init() {
	var off int64
	version := runtime.Version()
	for k, v := range offsetDictMap {
		if version == k || strings.HasPrefix(version, k) {
			off = v
			break
		}
	}
	offset = uintptr(off)
}

// getGoidByReflect parse the current goroutine's id from G.
// This function could be very fast(like 1ns/op), but it could be failed.
func getGoidByReflect() (bool, int64) {
	if offset == 0 {
		return false, 0
	}
	tmp := g.G()
	if tmp == nil {
		return false, 0
	}
	p := (*int64)(unsafe.Pointer(uintptr(tmp) + offset))
	return true, *p
}
