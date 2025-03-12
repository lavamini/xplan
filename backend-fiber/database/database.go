package database

import (
	"database/sql"

	"github.com/shirou/gopsutil/cpu"
)

var Db *sql.DB

// conn_str: user:password@tcp(host:port)/database?loc=Local&parseTime=true
func InitDatabase(connStr string) error {
	// 获取 cpu 核心数（不包括超线程）
	numCPU, _ := cpu.Counts(false)

	var err error
	Db, err = sql.Open("mysql", connStr)
	if err != nil {
		return err
	}
	// 设置连接池大小
	Db.SetMaxIdleConns(numCPU*2 + 1)
	Db.SetMaxOpenConns(numCPU*2 + 1)
	return nil
}
