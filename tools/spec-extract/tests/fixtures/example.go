package example

import (
	"errors"
	"io"
)

// ErrNotFound is returned when a resource is not found.
var ErrNotFound = errors.New("not found")

// MaxRetries is the maximum number of retry attempts.
const MaxRetries = 3

// User represents a user in the system.
type User struct {
	ID   int64
	Name string
	io.Reader // embedded
}

// Reader is an interface for reading data.
type Reader interface {
	io.Reader
	ReadAll() ([]byte, error)
}

// NewUser creates a new user with the given name.
func NewUser(name string) *User {
	return &User{Name: name}
}

// Greet returns a greeting for the user.
func (u *User) Greet() string {
	return "Hello, " + u.Name
}

// SetName updates the user's name.
func (u *User) SetName(name string) {
	u.Name = name
}
