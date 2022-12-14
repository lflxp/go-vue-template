package decode

import (
	"io"

	"gopkg.in/yaml.v2"
)

// YAMLDecode yaml decoder core data structure
type YAMLDecode struct {
	obj interface{}
}

// NewYAMLDecode create a new yaml decoder
func NewYAMLDecode(obj interface{}) Decoder {
	if obj == nil {
		return nil
	}
	return &YAMLDecode{obj: obj}
}

// Decode yaml decoder
func (y *YAMLDecode) Decode(r io.Reader) error {
	decode := yaml.NewDecoder(r)
	return decode.Decode(y.obj)
}

// Decode obj
func (y *YAMLDecode) Value() interface{} {
	return y.obj
}

// YAML yaml decoder
func YAML(r io.Reader, obj interface{}) error {
	decode := yaml.NewDecoder(r)
	return decode.Decode(obj)
}
