package models

// Group represents a collection of items with a name
type Group struct {
	Name  string   `yaml:"name"`
	Items []string `yaml:"items"`
}

// AppSetting represents the application configuration
type AppSetting struct {
	DefaultGroup *string `yaml:"default_group"`
	Groups       []Group `yaml:"groups"`
}

// Enums for list types
const (
	TypeAll      = "all"
	TypeChosen   = "chosen"
	TypeUnchosen = "unchosen"
)

// Helper function for string pointers
func StrPtr(s string) *string {
	return &s
}
