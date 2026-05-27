package database

import (
	"database/sql"
)

var Db *sql.DB

// conn_str: user:password@tcp(host:port)/database?loc=Local&parseTime=true
func InitDatabase(connStr string, min_connections int, max_connections int) error {
	var err error
	Db, err = sql.Open("mysql", connStr)
	if err != nil {
		return err
	}
	// 设置连接池大小
	Db.SetMaxIdleConns(min_connections)
	Db.SetMaxOpenConns(max_connections)
	return nil
}
