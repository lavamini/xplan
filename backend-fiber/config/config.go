package config

import "github.com/BurntSushi/toml"

type Db struct {
	Host           string
	Port           int
	User           string
	Password       string
	Database       string
	MinConnections int `toml:"min_connections"`
	MaxConnections int `toml:"max_connections"`
}

type Config struct {
	Db *Db
}

var config Config

func LoadConfig(path string) (Config, error) {
	_, err := toml.DecodeFile(path, &config)
	return config, err
}
